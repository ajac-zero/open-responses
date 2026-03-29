#!/usr/bin/env python3
"""
Validation script to check Rust models against OpenAPI schema.
Compares structs (field presence, types, requiredness), enums (variant coverage),
and resolves $ref / allOf / anyOf / oneOf in the spec.
"""

import json
import re
import sys
from pathlib import Path


# ──────────────────────────────────────────────────────────────────────
# 1. OpenAPI helpers
# ──────────────────────────────────────────────────────────────────────

def resolve_ref(spec, ref):
    """Follow a $ref string and return the resolved schema dict."""
    parts = ref.lstrip("#/").split("/")
    node = spec
    for p in parts:
        node = node[p]
    return node


def resolve_schema(spec, schema):
    """Resolve a schema that may use $ref, allOf, anyOf, or oneOf.
    Returns a 'flattened' dict with 'type', 'properties', 'required', 'enum',
    'oneOf', 'anyOf' keys as applicable.
    """
    if "$ref" in schema:
        return resolve_schema(spec, resolve_ref(spec, schema["$ref"]))

    # allOf: merge all sub-schemas
    if "allOf" in schema:
        merged = {"properties": {}, "required": []}
        for sub in schema["allOf"]:
            resolved = resolve_schema(spec, sub)
            merged["properties"].update(resolved.get("properties", {}))
            merged["required"].extend(resolved.get("required", []))
            for k in ("type", "enum"):
                if k in resolved:
                    merged[k] = resolved[k]
        return merged

    return schema


def get_json_type(spec, prop_schema):
    """Return the effective JSON type(s) for a property schema.
    Returns a set of type strings, e.g. {'string'}, {'integer', 'null'}.
    """
    prop_schema = resolve_schema(spec, prop_schema)
    types = set()

    if "type" in prop_schema:
        types.add(prop_schema["type"])
    if "enum" in prop_schema:
        types.add("string")  # enums are strings
    if "$ref" in prop_schema:
        resolved = resolve_ref(spec, prop_schema["$ref"])
        return get_json_type(spec, resolved)

    for key in ("anyOf", "oneOf"):
        if key in prop_schema:
            for sub in prop_schema[key]:
                types |= get_json_type(spec, sub)

    return types or {"unknown"}


def is_nullable(spec, prop_schema):
    """Check whether a property schema allows null."""
    for key in ("anyOf", "oneOf"):
        if key in prop_schema:
            for sub in prop_schema[key]:
                if sub.get("type") == "null":
                    return True
    if prop_schema.get("type") == "null":
        return True
    return False


# ──────────────────────────────────────────────────────────────────────
# 2. Rust parsing
# ──────────────────────────────────────────────────────────────────────

def parse_serde_attrs(attr_lines):
    """Parse serde attributes from a list of attribute lines above a field.
    Returns dict with keys: rename, skip_deserializing, skip_serializing_if, flatten, default.
    """
    attrs = {
        "rename": None,
        "skip_deserializing": False,
        "skip_serializing_if": False,
        "flatten": False,
        "default": False,
    }
    for line in attr_lines:
        m = re.search(r'#\[serde\((.+)\)\]', line)
        if not m:
            continue
        content = m.group(1)
        if "skip_deserializing" in content:
            attrs["skip_deserializing"] = True
        if "skip_serializing_if" in content:
            attrs["skip_serializing_if"] = True
        if "flatten" in content:
            attrs["flatten"] = True
        if re.search(r'\bdefault\b', content):
            attrs["default"] = True
        rename_m = re.search(r'rename\s*=\s*"([^"]+)"', content)
        if rename_m:
            attrs["rename"] = rename_m.group(1)
    return attrs


def extract_rust_structs(src_dir):
    """Extract all public structs with full field info from Rust files (recursive)."""
    rust_structs = {}
    src_path = Path(src_dir)

    for rs_file in src_path.rglob("*.rs"):
        with open(rs_file, errors="ignore") as f:
            content = f.read()

        # Find all struct blocks
        for m in re.finditer(r'pub struct (\w+)\s*\{([^}]*)\}', content, re.DOTALL):
            struct_name = m.group(1)
            body = m.group(2)
            fields = {}

            # Split body into lines and walk them, collecting serde attrs
            lines = body.split("\n")
            attr_lines = []
            for line in lines:
                stripped = line.strip()
                if stripped.startswith("#["):
                    attr_lines.append(stripped)
                    continue
                field_m = re.match(r'pub (\w+):\s*(.+?),?\s*$', stripped)
                if field_m:
                    field_name = field_m.group(1)
                    field_type = field_m.group(2).strip().rstrip(",")
                    serde = parse_serde_attrs(attr_lines)
                    json_name = serde["rename"] if serde["rename"] else field_name
                    fields[json_name] = {
                        "rust_name": field_name,
                        "rust_type": field_type,
                        "serde": serde,
                    }
                    attr_lines = []
                elif stripped.startswith("///") or stripped.startswith("//") or stripped == "":
                    # doc comments – keep accumulating attrs
                    pass
                else:
                    attr_lines = []

            rust_structs[struct_name] = {
                "fields": fields,
                "file": str(rs_file.relative_to(src_path.parent)),
            }

        # Also find type aliases (e.g., pub type Name = OtherType;)
        for m in re.finditer(r'pub type (\w+)\s*=\s*([^;]+);', content):
            type_name = m.group(1)
            # Treat type aliases as empty structs for matching purposes
            rust_structs[type_name] = {
                "fields": {},
                "file": str(rs_file.relative_to(src_path.parent)),
                "is_type_alias": True,
            }

    return rust_structs


def extract_rust_enums(src_dir):
    """Extract all public enums with variant info from Rust files (recursive)."""
    rust_enums = {}
    src_path = Path(src_dir)

    for rs_file in src_path.rglob("*.rs"):
        with open(rs_file, errors="ignore") as f:
            content = f.read()

        # Find enum-level serde attributes + enum block
        for m in re.finditer(
            r'((?:#\[(?:derive|serde)\([^\]]*\)\]\s*)*)'
            r'pub enum (\w+)\s*\{([^}]*)\}',
            content, re.DOTALL
        ):
            enum_attrs_block = m.group(1)
            enum_name = m.group(2)
            body = m.group(3)

            # Parse enum-level serde rename_all
            rename_all = None
            ra_m = re.search(r'serde\([^)]*rename_all\s*=\s*"([^"]+)"', enum_attrs_block)
            if ra_m:
                rename_all = ra_m.group(1)

            # Check if it's a tagged enum (has inner structs) vs simple string enum
            is_tagged = bool(re.search(r'\w+\s*\(', body))

            variants = []
            lines = body.split("\n")
            attr_lines = []
            for line in lines:
                stripped = line.strip()
                if stripped.startswith("#["):
                    attr_lines.append(stripped)
                    continue
                if stripped.startswith("///") or stripped.startswith("//") or stripped == "":
                    continue

                # Simple variant: Name or Name(Type)
                var_m = re.match(r'(\w+)(?:\s*\(([^)]*)\))?\s*,?\s*$', stripped)
                if var_m:
                    var_name = var_m.group(1)
                    var_type = var_m.group(2)
                    serde = parse_serde_attrs(attr_lines)
                    json_value = serde["rename"] if serde["rename"] else apply_rename_all(var_name, rename_all)
                    variants.append({
                        "rust_name": var_name,
                        "json_value": json_value,
                        "inner_type": var_type.strip() if var_type else None,
                    })
                    attr_lines = []
                else:
                    attr_lines = []

            rust_enums[enum_name] = {
                "variants": variants,
                "rename_all": rename_all,
                "is_tagged": is_tagged,
                "file": str(rs_file.relative_to(src_path.parent)),
            }

    return rust_enums


def apply_rename_all(variant_name, rename_all):
    """Apply serde rename_all to a PascalCase variant name."""
    if rename_all is None:
        return variant_name

    if rename_all == "lowercase":
        return variant_name.lower()
    elif rename_all == "UPPERCASE":
        return variant_name.upper()
    elif rename_all == "snake_case":
        # PascalCase -> snake_case
        s = re.sub(r'([A-Z])', r'_\1', variant_name).lstrip('_').lower()
        return s
    elif rename_all == "SCREAMING_SNAKE_CASE":
        s = re.sub(r'([A-Z])', r'_\1', variant_name).lstrip('_').upper()
        return s
    elif rename_all == "camelCase":
        return variant_name[0].lower() + variant_name[1:]
    elif rename_all == "PascalCase":
        return variant_name
    elif rename_all == "kebab-case":
        s = re.sub(r'([A-Z])', r'-\1', variant_name).lstrip('-').lower()
        return s
    return variant_name


# ──────────────────────────────────────────────────────────────────────
# 3. Rust type → JSON type mapping
# ──────────────────────────────────────────────────────────────────────

RUST_TO_JSON = {
    "String": {"string"},
    "bool": {"boolean"},
    "i8": {"integer"}, "i16": {"integer"}, "i32": {"integer"}, "i64": {"integer"},
    "u8": {"integer"}, "u16": {"integer"}, "u32": {"integer"}, "u64": {"integer"},
    "f32": {"number"}, "f64": {"number"},
    "serde_json::Value": {"string", "object", "array", "number", "integer", "boolean", "null", "unknown"},
}


def rust_type_to_json_types(rust_type):
    """Map a Rust type to the set of JSON types it can produce."""
    # Option<T> → nullable T
    opt_m = re.match(r'Option<(.+)>$', rust_type)
    if opt_m:
        inner = rust_type_to_json_types(opt_m.group(1))
        return inner | {"null"}

    # Vec<T> → array
    vec_m = re.match(r'Vec<(.+)>$', rust_type)
    if vec_m:
        return {"array"}

    # HashMap<K,V> → object
    if rust_type.startswith("HashMap<"):
        return {"object"}

    if rust_type in RUST_TO_JSON:
        return RUST_TO_JSON[rust_type]

    # Enum / struct types → could be string (for string enums) or object
    # We can't fully resolve without deeper analysis, so accept either
    return {"string", "object", "array", "unknown"}


# ──────────────────────────────────────────────────────────────────────
# 4. Validation
# ──────────────────────────────────────────────────────────────────────

def validate_structs(spec, rust_structs):
    """Validate Rust structs against OpenAPI object schemas."""
    schemas = spec["components"]["schemas"]
    issues = []

    for schema_name in sorted(schemas.keys()):
        schema = resolve_schema(spec, schemas[schema_name])
        if schema.get("type") != "object":
            continue
        if schema_name not in rust_structs:
            continue

        required = set(schema.get("required", []))
        props = schema.get("properties", {})
        prop_names = set(props.keys())
        rust_info = rust_structs[schema_name]
        rust_fields = rust_info["fields"]
        rust_json_names = set(rust_fields.keys())

        # a) Missing required fields
        for req in sorted(required):
            if req not in rust_json_names:
                issues.append({
                    "struct": schema_name,
                    "category": "missing_required",
                    "field": req,
                    "message": f"{schema_name}: Missing required field '{req}'",
                })

        # b) Extra fields in Rust not in spec
        extra = rust_json_names - prop_names
        for field in sorted(extra):
            issues.append({
                "struct": schema_name,
                "category": "extra_field",
                "field": field,
                "message": f"{schema_name}: Extra Rust field '{field}' (not in spec)",
            })

        # d) Missing optional fields in spec but not in Rust
        missing_optional = prop_names - rust_json_names - required
        for field in sorted(missing_optional):
            issues.append({
                "struct": schema_name,
                "category": "missing_optional_field",
                "field": field,
                "message": f"{schema_name}: Missing optional field '{field}' (in spec but not in Rust)",
            })

        # c) Type mismatches
        for prop_name in sorted(prop_names & rust_json_names):
            spec_types = get_json_type(spec, props[prop_name])
            rust_field = rust_fields[prop_name]
            rust_types = rust_type_to_json_types(rust_field["rust_type"])
            
            # serde_json::Value is flexible enough to handle any JSON type
            rust_is_json_value = "serde_json::Value" in rust_field["rust_type"]

            # Check if there's any overlap (skip for serde_json::Value which handles all types)
            if not rust_is_json_value and not (spec_types & rust_types):
                issues.append({
                    "struct": schema_name,
                    "category": "type_mismatch",
                    "field": prop_name,
                    "message": (
                        f"{schema_name}.{prop_name}: Type mismatch — "
                        f"spec expects {sorted(spec_types)}, "
                        f"Rust type '{rust_field['rust_type']}' maps to {sorted(rust_types)}"
                    ),
                })

            # Check nullability: if spec allows null, Rust should be Option
            spec_nullable = is_nullable(spec, props[prop_name])
            rust_is_option = rust_field["rust_type"].startswith("Option<")
            if spec_nullable and not rust_is_option and not rust_is_json_value:
                if prop_name not in required:
                    pass  # optional and not Option is fine if skip_serializing_if
                else:
                    issues.append({
                        "struct": schema_name,
                        "category": "nullability",
                        "field": prop_name,
                        "message": (
                            f"{schema_name}.{prop_name}: Spec allows null but "
                            f"Rust type is '{rust_field['rust_type']}' (not Option<T>)"
                        ),
                    })
            
            # Check if required field is incorrectly wrapped in Option
            # Required fields should not be Option unless spec explicitly allows null
            # Exception: serde_json::Value can handle null, so it's acceptable
            if prop_name in required and rust_is_option and not spec_nullable:
                # serde_json::Value is flexible enough to handle any type including null
                if not rust_is_json_value:
                    issues.append({
                        "struct": schema_name,
                        "category": "required_field_option",
                        "field": prop_name,
                        "message": (
                            f"{schema_name}.{prop_name}: Required field should not be Option<T> "
                            f"(spec does not allow null, but Rust type is '{rust_field['rust_type']}')"
                        ),
                    })

    return issues


def validate_enums(spec, rust_enums):
    """Validate Rust enums against OpenAPI enum schemas."""
    schemas = spec["components"]["schemas"]
    issues = []

    for schema_name in sorted(schemas.keys()):
        schema = schemas[schema_name]
        if "enum" not in schema:
            continue
        if schema_name not in rust_enums:
            continue

        spec_values = set(schema["enum"])
        rust_enum = rust_enums[schema_name]
        rust_values = {v["json_value"] for v in rust_enum["variants"]}

        missing = spec_values - rust_values
        extra = rust_values - spec_values

        for v in sorted(missing):
            issues.append({
                "enum": schema_name,
                "category": "missing_variant",
                "variant": v,
                "message": f"{schema_name}: Missing enum variant '{v}' (in spec but not in Rust)",
            })
        for v in sorted(extra):
            issues.append({
                "enum": schema_name,
                "category": "extra_variant",
                "variant": v,
                "message": f"{schema_name}: Extra Rust variant '{v}' (not in spec)",
            })

    return issues


def find_schema_references(spec):
    """Find all $ref references to schemas in the spec.
    Returns a set of schema names that are referenced.
    """
    referenced = set()
    
    def find_refs(obj):
        if isinstance(obj, dict):
            if "$ref" in obj:
                ref = obj["$ref"]
                if ref.startswith("#/components/schemas/"):
                    schema_name = ref.split("/")[-1]
                    referenced.add(schema_name)
            for v in obj.values():
                find_refs(v)
        elif isinstance(obj, list):
            for item in obj:
                find_refs(item)
    
    find_refs(spec)
    return referenced


def find_unmatched(spec, rust_structs, rust_enums):
    """Find OpenAPI schemas with no corresponding Rust type.
    Categorizes into 'used' (referenced elsewhere in spec) and 'unused' (dead code in spec).
    """
    schemas = spec["components"]["schemas"]
    referenced = find_schema_references(spec)
    unmatched_used = []
    unmatched_unused = []

    for schema_name in sorted(schemas.keys()):
        schema = schemas[schema_name]
        if schema_name in rust_structs or schema_name in rust_enums:
            continue
        # Skip pure union types (oneOf/anyOf without properties) — these are
        # typically represented as Rust enums with different names
        if schema.get("type") not in ("object", "string"):
            if "enum" not in schema and "properties" not in schema:
                continue
        
        if schema_name in referenced:
            unmatched_used.append(schema_name)
        else:
            unmatched_unused.append(schema_name)

    return unmatched_used, unmatched_unused


# ──────────────────────────────────────────────────────────────────────
# 5. Reporting
# ──────────────────────────────────────────────────────────────────────

def categorize_issues(issues):
    """Categorize issues by struct type."""
    core = []
    streaming = []
    param = []
    for i in issues:
        name = i.get("struct") or i.get("enum", "")
        if "Streaming" in name:
            streaming.append(i)
        elif "Param" in name:
            param.append(i)
        else:
            core.append(i)
    return {"core": core, "streaming": streaming, "param": param, "total": len(issues)}


def print_report(categorized, unmatched_used, unmatched_unused, *, show_streaming=False, show_params=False, show_all=False):
    """Print validation report."""
    total = categorized["total"]

    if total == 0 and not unmatched_used and not unmatched_unused:
        print("✅ All models validated successfully!")
        return

    print(f"\n{'=' * 70}")
    print(f"VALIDATION REPORT — {total} issues found")
    print(f"{'=' * 70}\n")

    def print_section(title, items, always_show=True):
        if not items:
            return
        if always_show:
            print(f"{title} ({len(items)} issues):")
            # Group by category
            by_cat = {}
            for i in items:
                by_cat.setdefault(i["category"], []).append(i)
            for cat in ("missing_required", "required_field_option", "missing_optional_field", "extra_field", "type_mismatch",
                        "nullability", "missing_variant", "extra_variant"):
                if cat not in by_cat:
                    continue
                cat_label = cat.replace("_", " ").title()
                print(f"\n  [{cat_label}]")
                for issue in sorted(by_cat[cat], key=lambda x: x["message"]):
                    print(f"    ❌ {issue['message']}")
            print()
        else:
            print(f"{title} ({len(items)} issues)")
            print(f"  (Run with --show-all to see details)\n")

    print_section("CORE ISSUES", categorized["core"])
    print_section("STREAMING EVENTS", categorized["streaming"],
                  always_show=show_streaming or show_all)
    print_section("PARAM VARIANTS", categorized["param"],
                  always_show=show_params or show_all)

    if unmatched_used:
        print(f"MISSING TYPES ({len(unmatched_used)} schemas used in spec but missing in Rust):")
        for name in unmatched_used:
            print(f"  ❌ {name}")
        print()

    if unmatched_unused:
        print(f"UNUSED SPEC SCHEMAS ({len(unmatched_unused)} schemas defined in spec but never referenced):")
        for name in unmatched_unused:
            print(f"  ⚠️  {name} (dead code in spec)")
        print()


# ──────────────────────────────────────────────────────────────────────
# 6. Main
# ──────────────────────────────────────────────────────────────────────

def main():
    import argparse

    parser = argparse.ArgumentParser(description="Validate Rust models against OpenAPI schema")
    parser.add_argument("--openapi", default="openapi.json", help="Path to OpenAPI spec")
    parser.add_argument("--src", default="src", help="Path to Rust source directory")
    parser.add_argument("--show-streaming", action="store_true", help="Show streaming event issues")
    parser.add_argument("--show-params", action="store_true", help="Show parameter variant issues")
    parser.add_argument("--show-all", action="store_true", help="Show all issues")
    args = parser.parse_args()

    with open(args.openapi) as f:
        spec = json.load(f)

    rust_structs = extract_rust_structs(args.src)
    rust_enums = extract_rust_enums(args.src)

    print(f"Parsed {len(rust_structs)} structs and {len(rust_enums)} enums from Rust source\n")

    struct_issues = validate_structs(spec, rust_structs)
    enum_issues = validate_enums(spec, rust_enums)
    all_issues = struct_issues + enum_issues

    categorized = categorize_issues(all_issues)
    unmatched_used, unmatched_unused = find_unmatched(spec, rust_structs, rust_enums)

    print_report(
        categorized,
        unmatched_used,
        unmatched_unused,
        show_streaming=args.show_streaming,
        show_params=args.show_params,
        show_all=args.show_all,
    )

    return 0 if categorized["total"] == 0 and not unmatched_used else 1


if __name__ == "__main__":
    sys.exit(main())
