#!/usr/bin/env python3
"""
Validation script to check Rust models against OpenAPI schema.
Compares required fields, properties, and detects mismatches.
"""

import json
import re
import sys
from pathlib import Path

def extract_rust_structs(src_dir):
    """Extract all public structs and their fields from Rust files."""
    rust_structs = {}
    src_path = Path(src_dir)
    
    for rs_file in src_path.glob('*.rs'):
        with open(rs_file, errors='ignore') as f:
            content = f.read()
            matches = re.findall(r'pub struct (\w+)\s*\{([^}]*)\}', content, re.DOTALL)
            for name, body in matches:
                fields = re.findall(r'pub (\w+):\s*([^,\n]+)', body)
                rust_structs[name] = {field: type_.strip() for field, type_ in fields}
    
    return rust_structs

def validate_models(openapi_path, src_dir):
    """Validate Rust models against OpenAPI schema."""
    
    with open(openapi_path) as f:
        spec = json.load(f)
    
    schemas = spec['components']['schemas']
    rust_structs = extract_rust_structs(src_dir)
    
    issues = []
    
    # Check all object schemas
    for schema_name in sorted(schemas.keys()):
        if not schema_name[0].isupper():
            continue
        
        schema = schemas[schema_name]
        if schema.get('type') != 'object':
            continue
        
        required = set(schema.get('required', []))
        props = set(schema.get('properties', {}).keys())
        
        # Skip if no matching Rust struct
        if schema_name not in rust_structs:
            continue
        
        rust_fields = set(rust_structs[schema_name].keys())
        
        # Check for missing required fields
        for req in required:
            if req == 'type':
                if 'type_' not in rust_fields:
                    issues.append({
                        'struct': schema_name,
                        'type': 'missing_required',
                        'field': 'type',
                        'message': f"{schema_name}: Missing required 'type' field"
                    })
            else:
                if req not in rust_fields:
                    issues.append({
                        'struct': schema_name,
                        'type': 'missing_required',
                        'field': req,
                        'message': f"{schema_name}: Missing required field '{req}'"
                    })
        
        # Check for extra fields in Rust that aren't in OpenAPI
        extra_fields = rust_fields - props - {'type_'}
        for extra in sorted(extra_fields):
            issues.append({
                'struct': schema_name,
                'type': 'extra_field',
                'field': extra,
                'message': f"{schema_name}: Extra Rust field '{extra}' (not in spec)"
            })
    
    return issues

def categorize_issues(issues):
    """Categorize issues by type and severity."""
    core_issues = [i for i in issues if 'Streaming' not in i['struct'] and 'Param' not in i['struct']]
    streaming_issues = [i for i in issues if 'Streaming' in i['struct']]
    param_issues = [i for i in issues if 'Param' in i['struct']]
    
    return {
        'core': core_issues,
        'streaming': streaming_issues,
        'param': param_issues,
        'total': len(issues)
    }

def print_report(issues_by_category):
    """Print validation report."""
    
    if issues_by_category['total'] == 0:
        print("✅ All models validated successfully!")
        return
    
    print(f"\n{'='*70}")
    print(f"VALIDATION REPORT - {issues_by_category['total']} issues found")
    print(f"{'='*70}\n")
    
    if issues_by_category['core']:
        print(f"CORE ISSUES ({len(issues_by_category['core'])} issues):")
        for issue in sorted(issues_by_category['core'], key=lambda x: x['struct']):
            print(f"  ❌ {issue['message']}")
        print()
    
    if issues_by_category['streaming']:
        print(f"STREAMING EVENTS ({len(issues_by_category['streaming'])} issues)")
        print("  (Run with --show-streaming to see details)\n")
    
    if issues_by_category['param']:
        print(f"PARAM VARIANTS ({len(issues_by_category['param'])} issues)")
        print("  (Run with --show-params to see details)\n")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='Validate Rust models against OpenAPI schema')
    parser.add_argument('--openapi', default='openapi.json', help='Path to OpenAPI spec')
    parser.add_argument('--src', default='src', help='Path to Rust source directory')
    parser.add_argument('--show-streaming', action='store_true', help='Show streaming event issues')
    parser.add_argument('--show-params', action='store_true', help='Show parameter variant issues')
    parser.add_argument('--show-all', action='store_true', help='Show all issues')
    args = parser.parse_args()
    
    issues = validate_models(args.openapi, args.src)
    categorized = categorize_issues(issues)
    
    print_report(categorized)
    
    if args.show_all or args.show_streaming:
        if categorized['streaming']:
            print(f"STREAMING EVENT ISSUES ({len(categorized['streaming'])} issues):")
            for issue in sorted(categorized['streaming'], key=lambda x: x['struct']):
                print(f"  ❌ {issue['message']}")
            print()
    
    if args.show_all or args.show_params:
        if categorized['param']:
            print(f"PARAMETER VARIANT ISSUES ({len(categorized['param'])} issues):")
            for issue in sorted(categorized['param'], key=lambda x: x['struct']):
                print(f"  ❌ {issue['message']}")
            print()
    
    return 0 if categorized['total'] == 0 else 1

if __name__ == '__main__':
    sys.exit(main())
