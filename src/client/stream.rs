use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use crate::client::{Error, Result};
use crate::{
    Annotation, ContentPart, ErrorStreamingEvent, OutputItem, ReasoningContentPart,
    ResponseCompletedStreamingEvent, ResponseContentPartAddedStreamingEvent,
    ResponseContentPartDoneStreamingEvent, ResponseCreatedStreamingEvent,
    ResponseFailedStreamingEvent, ResponseFunctionCallArgumentsDeltaStreamingEvent,
    ResponseFunctionCallArgumentsDoneStreamingEvent, ResponseInProgressStreamingEvent,
    ResponseIncompleteStreamingEvent, ResponseOutputItemAddedStreamingEvent,
    ResponseOutputItemDoneStreamingEvent, ResponseOutputTextAnnotationAddedStreamingEvent,
    ResponseOutputTextDeltaStreamingEvent, ResponseOutputTextDoneStreamingEvent,
    ResponseQueuedStreamingEvent, ResponseReasoningDeltaStreamingEvent,
    ResponseReasoningDoneStreamingEvent, ResponseReasoningSummaryDeltaStreamingEvent,
    ResponseReasoningSummaryDoneStreamingEvent, ResponseReasoningSummaryPartAddedStreamingEvent,
    ResponseReasoningSummaryPartDoneStreamingEvent, ResponseRefusalDeltaStreamingEvent,
    ResponseRefusalDoneStreamingEvent, ResponseResource,
};

/// All possible streaming events from the Open Responses API.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ResponseEvent {
    Created {
        response: ResponseResource,
    },
    InProgress {
        response: ResponseResource,
    },
    Queued {
        response: ResponseResource,
    },
    OutputItemAdded {
        output_index: i64,
        item: Option<OutputItem>,
    },
    OutputItemDone {
        output_index: i64,
        item: Option<OutputItem>,
    },
    ContentPartAdded {
        item_id: String,
        output_index: i64,
        content_index: i64,
        part: ContentPart,
    },
    ContentPartDone {
        item_id: String,
        output_index: i64,
        content_index: i64,
        part: ContentPart,
    },
    TextDelta {
        item_id: String,
        output_index: i64,
        content_index: i64,
        delta: String,
    },
    TextDone {
        item_id: String,
        output_index: i64,
        content_index: i64,
        text: String,
    },
    TextAnnotationAdded {
        item_id: String,
        output_index: i64,
        content_index: i64,
        annotation_index: i64,
        annotation: Option<Annotation>,
    },
    FunctionCallDelta {
        item_id: String,
        output_index: i64,
        delta: String,
    },
    FunctionCallDone {
        item_id: String,
        output_index: i64,
        arguments: String,
    },
    ReasoningDelta {
        item_id: String,
        output_index: i64,
        content_index: i64,
        delta: String,
    },
    ReasoningDone {
        item_id: String,
        output_index: i64,
        content_index: i64,
        text: String,
    },
    ReasoningSummaryDelta {
        item_id: String,
        output_index: i64,
        summary_index: i64,
        delta: String,
    },
    ReasoningSummaryDone {
        item_id: String,
        output_index: i64,
        summary_index: i64,
        text: String,
    },
    ReasoningSummaryPartAdded {
        item_id: String,
        output_index: i64,
        summary_index: i64,
        part: ReasoningContentPart,
    },
    ReasoningSummaryPartDone {
        item_id: String,
        output_index: i64,
        summary_index: i64,
        part: ReasoningContentPart,
    },
    RefusalDelta {
        item_id: String,
        output_index: i64,
        content_index: i64,
        delta: String,
    },
    RefusalDone {
        item_id: String,
        output_index: i64,
        content_index: i64,
        refusal: String,
    },
    Completed {
        response: ResponseResource,
    },
    Failed {
        response: ResponseResource,
    },
    Incomplete {
        response: ResponseResource,
    },
    Error {
        code: String,
        message: String,
    },
    Unknown {
        event_type: String,
        data: String,
    },
}

impl ResponseEvent {
    pub fn as_text_delta(&self) -> Option<&str> {
        match self {
            ResponseEvent::TextDelta { delta, .. } => Some(delta),
            _ => None,
        }
    }

    pub fn as_function_call_delta(&self) -> Option<&str> {
        match self {
            ResponseEvent::FunctionCallDelta { delta, .. } => Some(delta),
            _ => None,
        }
    }

    pub fn as_completed(&self) -> Option<&ResponseResource> {
        match self {
            ResponseEvent::Completed { response } => Some(response),
            _ => None,
        }
    }
}

fn parse_event(event_type: &str, data: &str) -> Result<ResponseEvent> {
    let event = match event_type {
        "response.created" => {
            let e: ResponseCreatedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Created {
                response: e.response,
            }
        }
        "response.in_progress" => {
            let e: ResponseInProgressStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::InProgress {
                response: e.response,
            }
        }
        "response.queued" => {
            let e: ResponseQueuedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Queued {
                response: e.response,
            }
        }
        "response.output_item.added" => {
            let e: ResponseOutputItemAddedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::OutputItemAdded {
                output_index: e.output_index,
                item: e.item,
            }
        }
        "response.output_item.done" => {
            let e: ResponseOutputItemDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::OutputItemDone {
                output_index: e.output_index,
                item: e.item,
            }
        }
        "response.content_part.added" => {
            let e: ResponseContentPartAddedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ContentPartAdded {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                part: e.part,
            }
        }
        "response.content_part.done" => {
            let e: ResponseContentPartDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ContentPartDone {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                part: e.part,
            }
        }
        "response.output_text.delta" => {
            let e: ResponseOutputTextDeltaStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::TextDelta {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                delta: e.delta,
            }
        }
        "response.output_text.done" => {
            let e: ResponseOutputTextDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::TextDone {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                text: e.text,
            }
        }
        "response.output_text.annotation.added" => {
            let e: ResponseOutputTextAnnotationAddedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::TextAnnotationAdded {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                annotation_index: e.annotation_index,
                annotation: e.annotation,
            }
        }
        "response.function_call_arguments.delta" => {
            let e: ResponseFunctionCallArgumentsDeltaStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::FunctionCallDelta {
                item_id: e.item_id,
                output_index: e.output_index,
                delta: e.delta,
            }
        }
        "response.function_call_arguments.done" => {
            let e: ResponseFunctionCallArgumentsDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::FunctionCallDone {
                item_id: e.item_id,
                output_index: e.output_index,
                arguments: e.arguments,
            }
        }
        "response.reasoning.delta" => {
            let e: ResponseReasoningDeltaStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningDelta {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                delta: e.delta,
            }
        }
        "response.reasoning.done" => {
            let e: ResponseReasoningDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningDone {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                text: e.text,
            }
        }
        "response.reasoning_summary.delta" => {
            let e: ResponseReasoningSummaryDeltaStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningSummaryDelta {
                item_id: e.item_id,
                output_index: e.output_index,
                summary_index: e.summary_index,
                delta: e.delta,
            }
        }
        "response.reasoning_summary.done" => {
            let e: ResponseReasoningSummaryDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningSummaryDone {
                item_id: e.item_id,
                output_index: e.output_index,
                summary_index: e.summary_index,
                text: e.text,
            }
        }
        "response.reasoning_summary_part.added" => {
            let e: ResponseReasoningSummaryPartAddedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningSummaryPartAdded {
                item_id: e.item_id,
                output_index: e.output_index,
                summary_index: e.summary_index,
                part: e.part,
            }
        }
        "response.reasoning_summary_part.done" => {
            let e: ResponseReasoningSummaryPartDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::ReasoningSummaryPartDone {
                item_id: e.item_id,
                output_index: e.output_index,
                summary_index: e.summary_index,
                part: e.part,
            }
        }
        "response.refusal.delta" => {
            let e: ResponseRefusalDeltaStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::RefusalDelta {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                delta: e.delta,
            }
        }
        "response.refusal.done" => {
            let e: ResponseRefusalDoneStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::RefusalDone {
                item_id: e.item_id,
                output_index: e.output_index,
                content_index: e.content_index,
                refusal: e.refusal,
            }
        }
        "response.completed" => {
            let e: ResponseCompletedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Completed {
                response: e.response,
            }
        }
        "response.failed" => {
            let e: ResponseFailedStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Failed {
                response: e.response,
            }
        }
        "response.incomplete" => {
            let e: ResponseIncompleteStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Incomplete {
                response: e.response,
            }
        }
        "error" => {
            let e: ErrorStreamingEvent = serde_json::from_str(data)?;
            ResponseEvent::Error {
                code: e.error.code,
                message: e.error.message,
            }
        }
        other => ResponseEvent::Unknown {
            event_type: other.to_string(),
            data: data.to_string(),
        },
    };

    Ok(event)
}

struct ResponseStreamCore {
    events: tokio::sync::mpsc::UnboundedReceiver<Result<ResponseEvent>>,
    result: Option<tokio::sync::oneshot::Receiver<Result<ResponseResource>>>,
}

impl ResponseStreamCore {
    fn spawn(response: reqwest::Response, rt: Option<Arc<tokio::runtime::Runtime>>) -> Self {
        use futures_core::Stream as FuturesStream;
        use std::pin::pin;

        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();

        let producer = async move {
            let mut stream = pin!(response.bytes_stream());
            let mut buffer = Vec::new();
            let mut result_tx = Some(result_tx);

            loop {
                if let Some((event_type, data)) = extract_sse_frame(&mut buffer) {
                    if data == "[DONE]" {
                        break;
                    }

                    let parsed = resolve_event_type(event_type.as_deref(), &data);

                    match &parsed {
                        Ok(ResponseEvent::Completed { response }) => {
                            if let Some(tx) = result_tx.take() {
                                let _ = tx.send(Ok(response.clone()));
                            }
                        }
                        Ok(ResponseEvent::Failed { response }) => {
                            if let Some(tx) = result_tx.take() {
                                let _ = tx.send(Err(Error::StreamClosed));
                                let _ = event_tx.send(Ok(ResponseEvent::Failed {
                                    response: response.clone(),
                                }));
                                break;
                            }
                        }
                        Ok(ResponseEvent::Incomplete { response }) => {
                            if let Some(tx) = result_tx.take() {
                                let _ = tx.send(Err(Error::StreamClosed));
                                let _ = event_tx.send(Ok(ResponseEvent::Incomplete {
                                    response: response.clone(),
                                }));
                                break;
                            }
                        }
                        _ => {}
                    }

                    if event_tx.send(parsed).is_err() {
                        break;
                    }

                    continue;
                }

                let next = std::future::poll_fn(|cx: &mut std::task::Context<'_>| {
                    stream.as_mut().poll_next(cx)
                })
                .await;

                match next {
                    None => break,
                    Some(Ok(bytes)) => buffer.extend_from_slice(&bytes),
                    Some(Err(e)) => {
                        let _ = event_tx.send(Err(Error::Reqwest(e)));
                        break;
                    }
                }
            }
        };

        match &rt {
            Some(handle) => {
                handle.spawn(producer);
            }
            None => {
                tokio::spawn(producer);
            }
        }

        Self {
            events: event_rx,
            result: Some(result_rx),
        }
    }

    async fn final_result(mut self) -> Result<ResponseResource> {
        while self.events.recv().await.is_some() {}

        self.result
            .take()
            .expect("final_result called only once")
            .await
            .map_err(|_| Error::StreamClosed)?
    }
}

/// A blocking streaming response from the Open Responses API.
pub struct ResponseStream {
    core: ResponseStreamCore,
    rt: Arc<tokio::runtime::Runtime>,
}

impl ResponseStream {
    pub(crate) fn spawn(response: reqwest::Response, rt: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            core: ResponseStreamCore::spawn(response, Some(rt.clone())),
            rt,
        }
    }

    /// Consumes the stream and returns the final response.
    /// Drains all remaining events before returning.
    pub fn final_result(self) -> Result<ResponseResource> {
        let Self { core, rt } = self;
        rt.block_on(core.final_result())
    }
}

/// An async streaming response from the Open Responses API.
pub struct AsyncResponseStream {
    core: ResponseStreamCore,
}

impl AsyncResponseStream {
    pub(crate) fn spawn(response: reqwest::Response) -> Self {
        Self {
            core: ResponseStreamCore::spawn(response, None),
        }
    }

    /// Consumes the stream and returns the final response.
    /// Drains all remaining events before returning.
    pub async fn final_result(self) -> Result<ResponseResource> {
        self.core.final_result().await
    }
}

fn extract_sse_frame(buffer: &mut Vec<u8>) -> Option<(Option<String>, String)> {
    let buf_str = String::from_utf8_lossy(buffer);

    let frame_end = buf_str
        .find("\n\n")
        .map(|pos| (pos, pos + 2))
        .or_else(|| buf_str.find("\r\n\r\n").map(|pos| (pos, pos + 4)));

    let (content_end, drain_end) = frame_end?;
    let frame = buf_str[..content_end].to_string();
    buffer.drain(..drain_end);

    let mut event_type = None;
    let mut data_lines = Vec::new();

    for line in frame.lines() {
        if let Some(value) = line.strip_prefix("event:") {
            event_type = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data:") {
            data_lines.push(value.trim_start().to_string());
        }
    }

    if data_lines.is_empty() {
        None
    } else {
        Some((event_type, data_lines.join("\n")))
    }
}

fn resolve_event_type(event_type: Option<&str>, data: &str) -> Result<ResponseEvent> {
    match event_type {
        Some(event_type) => parse_event(event_type, data),
        None => {
            let value: serde_json::Value = serde_json::from_str(data)?;
            let event_type = value
                .get("type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            parse_event(event_type, data)
        }
    }
}

impl futures_core::Stream for AsyncResponseStream {
    type Item = Result<ResponseEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.as_mut().get_mut().core.events.poll_recv(cx)
    }
}

impl Iterator for ResponseStream {
    type Item = Result<ResponseEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rt.block_on(self.core.events.recv())
    }
}

impl std::fmt::Debug for ResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseStream").finish()
    }
}

impl std::fmt::Debug for AsyncResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncResponseStream").finish()
    }
}
