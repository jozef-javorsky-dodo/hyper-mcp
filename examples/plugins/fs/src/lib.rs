mod pdk;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

use extism_pdk::*;
use json::Value;
use pdk::types::{
    CallToolRequest, CallToolResult, Content, ContentType, ListToolsResult, ToolDescription,
};
use serde_json::json;

pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    info!("call: {:?}", input);
    match input.params.name.as_str() {
        "read_file" => read_file(input),
        "read_multiple_files" => read_multiple_files(input),
        "write_file" => write_file(input),
        "edit_file" => edit_file(input),
        "create_dir" => create_dir(input),
        "list_dir" => list_dir(input),
        "move_file" => move_file(input),
        "search_files" => search_files(input),
        "get_file_info" => get_file_info(input),
        _ => Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some(format!("Unknown operation: {}", input.params.name)),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        }),
    }
}

fn read_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let Some(Value::String(path)) = args.get("path") {
        match fs::read_to_string(path) {
            Ok(content) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(content),
                    mime_type: Some("text/plain".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to read file: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide a path".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn read_multiple_files(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let Some(Value::Array(paths)) = args.get("paths") {
        let mut results = Vec::new();
        for path in paths {
            if let Value::String(path_str) = path {
                match fs::read_to_string(path_str) {
                    Ok(content) => results.push(json!({
                        "path": path_str,
                        "content": content,
                        "error": null
                    })),
                    Err(e) => results.push(json!({
                        "path": path_str,
                        "content": null,
                        "error": e.to_string()
                    })),
                }
            }
        }
        Ok(CallToolResult {
            is_error: None,
            content: vec![Content {
                annotations: None,
                text: Some(serde_json::to_string(&results)?),
                mime_type: Some("application/json".to_string()),
                r#type: ContentType::Text,
                data: None,
            }],
        })
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide an array of paths".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn write_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let (Some(Value::String(path)), Some(Value::String(content))) = (
        args.get("path"),
        args.get("content"),
    ) {
        match fs::write(path, content) {
            Ok(_) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some("File written successfully".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to write file: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide path and content".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn edit_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let (Some(Value::String(path)), Some(Value::String(content))) = (
        args.get("path"),
        args.get("content"),
    ) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(CallToolResult {
            is_error: None,
            content: vec![Content {
                annotations: None,
                text: Some("File edited successfully".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide path and content".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn create_dir(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let Some(Value::String(path)) = args.get("path") {
        match fs::create_dir_all(path) {
            Ok(_) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some("Directory created successfully".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to create directory: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide a path".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn list_dir(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let Some(Value::String(path)) = args.get("path") {
        match fs::read_dir(path) {
            Ok(entries) => {
                let mut items = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let metadata = entry.metadata()?;
                        items.push(json!({
                            "name": entry.file_name().to_string_lossy(),
                            "path": path.to_string_lossy(),
                            "is_file": metadata.is_file(),
                            "is_dir": metadata.is_dir(),
                            "size": metadata.len(),
                            "modified": metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()
                        }));
                    }
                }
                Ok(CallToolResult {
                    is_error: None,
                    content: vec![Content {
                        annotations: None,
                        text: Some(serde_json::to_string(&items)?),
                        mime_type: Some("application/json".to_string()),
                        r#type: ContentType::Text,
                        data: None,
                    }],
                })
            }
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to list directory: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide a path".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn move_file(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let (Some(Value::String(from)), Some(Value::String(to))) = (
        args.get("from"),
        args.get("to"),
    ) {
        match fs::rename(from, to) {
            Ok(_) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some("File moved successfully".into()),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to move file: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide from and to paths".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn search_files(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let (Some(Value::String(dir)), Some(Value::String(pattern))) = (
        args.get("directory"),
        args.get("pattern"),
    ) {
        let mut results = Vec::new();
        fn search_dir(dir: &Path, pattern: &str, results: &mut Vec<String>) -> io::Result<()> {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    search_dir(&path, pattern, results)?;
                } else if path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .contains(pattern)
                {
                    results.push(path.to_string_lossy().into_owned());
                }
            }
            Ok(())
        }
        match search_dir(Path::new(dir), pattern, &mut results) {
            Ok(_) => Ok(CallToolResult {
                is_error: None,
                content: vec![Content {
                    annotations: None,
                    text: Some(serde_json::to_string(&results)?),
                    mime_type: Some("application/json".to_string()),
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to search files: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide directory and pattern".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

fn get_file_info(input: CallToolRequest) -> Result<CallToolResult, Error> {
    let args = input.params.arguments.clone().unwrap_or_default();
    if let Some(Value::String(path)) = args.get("path") {
        match fs::metadata(path) {
            Ok(metadata) => {
                let info = json!({
                    "size": metadata.len(),
                    "is_file": metadata.is_file(),
                    "is_dir": metadata.is_dir(),
                    "modified": metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                    "created": metadata.created()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                    "accessed": metadata.accessed()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                });
                Ok(CallToolResult {
                    is_error: None,
                    content: vec![Content {
                        annotations: None,
                        text: Some(serde_json::to_string(&info)?),
                        mime_type: Some("application/json".to_string()),
                        r#type: ContentType::Text,
                        data: None,
                    }],
                })
            }
            Err(e) => Ok(CallToolResult {
                is_error: Some(true),
                content: vec![Content {
                    annotations: None,
                    text: Some(format!("Failed to get file info: {}", e)),
                    mime_type: None,
                    r#type: ContentType::Text,
                    data: None,
                }],
            }),
        }
    } else {
        Ok(CallToolResult {
            is_error: Some(true),
            content: vec![Content {
                annotations: None,
                text: Some("Please provide a path".into()),
                mime_type: None,
                r#type: ContentType::Text,
                data: None,
            }],
        })
    }
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    Ok(ListToolsResult {
        tools: vec![
            ToolDescription {
                name: "read_file".into(),
                description: "Read the contents of a file".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read",
                        },
                    },
                    "required": ["path"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "read_multiple_files".into(),
                description: "Read contents of multiple files".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "paths": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Array of file paths to read",
                        },
                    },
                    "required": ["paths"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "write_file".into(),
                description: "Write content to a file".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path where to write the file",
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write to the file",
                        },
                    },
                    "required": ["path", "content"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "edit_file".into(),
                description: "Edit an existing file's content".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to edit",
                        },
                        "content": {
                            "type": "string",
                            "description": "New content for the file",
                        },
                    },
                    "required": ["path", "content"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "create_dir".into(),
                description: "Create a new directory".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path where to create the directory",
                        },
                    },
                    "required": ["path"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "list_dir".into(),
                description: "List contents of a directory".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the directory to list",
                        },
                    },
                    "required": ["path"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "move_file".into(),
                description: "Move a file from one location to another".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "string",
                            "description": "Source path of the file",
                        },
                        "to": {
                            "type": "string",
                            "description": "Destination path for the file",
                        },
                    },
                    "required": ["from", "to"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "search_files".into(),
                description: "Search for files matching a pattern in a directory".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "directory": {
                            "type": "string",
                            "description": "Directory to search in",
                        },
                        "pattern": {
                            "type": "string",
                            "description": "Pattern to match against filenames",
                        },
                    },
                    "required": ["directory", "pattern"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            ToolDescription {
                name: "get_file_info".into(),
                description: "Get information about a file or directory".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to get information about",
                        },
                    },
                    "required": ["path"],
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        ],
    })
}
