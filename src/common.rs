
use crate::error::{KvsError, Result};
use bytes::Buf;

#[derive(Debug)]
pub enum RequestType {
    Get = 0x1,
    Put = 0x2,
    Delete = 0x3,
}

#[derive(Debug)]
pub enum ReplyType {
    Error = 0x1,
    Ok = 0x2,
    Msg = 0x3,
}

fn parse_request_type(t : u8) -> Result<RequestType> {
    match t {
        0x1 => Ok(RequestType::Get),
        0x2 => Ok(RequestType::Put),
        0x3 => Ok(RequestType::Delete),
        _ => Err(KvsError::InvalidRequest),
    }
}

fn parse_reply_type(t : u8) -> Result<ReplyType> {
    match t {
        0x1 => Ok(ReplyType::Error),
        0x2 => Ok(ReplyType::Ok),
        0x3 => Ok(ReplyType::Msg),
        _ => Err(KvsError::InvalidReply),
    }
}

#[derive(Debug)]
pub struct RequestMsg {
    pub request_type: RequestType,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct ReplyMsg {
    pub reply_type: ReplyType,
    pub value: Option<String>,
}

impl RequestMsg {
    pub fn build(request_type: RequestType, key : String, value : Option<String>) -> Vec<u8> {
        let mut buf = Vec::new();
        
        buf.push(request_type as u8);
        let key_len = key.len() as u32;
        buf.append(&mut key_len.to_be_bytes().to_vec());
        buf.append(&mut key.as_bytes().to_vec());

        if let Some(value) = value {
            let value_len = value.len() as u32;
            buf.append(&mut value_len.to_be_bytes().to_vec());
            buf.append(&mut value.as_bytes().to_vec());
        }

        buf
    }

    pub fn parse(buf: &[u8]) -> Result<RequestMsg> {
        let request_type = parse_request_type(buf[0])?;

        let mut index = 1;
        let key_len = (&buf[index..index + 4]).get_u32() as usize;
        index += 4;
        let key = String::from_utf8(buf[index..index + key_len].to_vec()).map_err(|_| {
            KvsError::InvalidRequest
        })?;
        index += key_len;

        match request_type {
            RequestType::Put => {
                let value_len = (&buf[index..index + 4]).get_u32() as usize;
                index += 4;
                let value = String::from_utf8(buf[index..index + value_len].to_vec()).map_err(|_| {
                    KvsError::InvalidRequest
                })?;

                Ok(RequestMsg {
                    request_type,
                    key,
                    value : Some(value),
                })
            }
            _ => {
                Ok(RequestMsg {
                    request_type,
                    key,
                    value : None,
                })
            }
        }

    }
}


impl ReplyMsg {
    pub fn build(self) ->  Vec<u8> {
        let mut buf = vec![];
        buf.push(self.reply_type as u8);
        if let Some(value) = self.value {
            let value_len = value.len() as u32;
            buf.append(&mut value_len.to_be_bytes().to_vec());
            buf.append(&mut value.as_bytes().to_vec());
        }
        buf
    }

    pub fn parse(buf: &[u8]) -> Result<ReplyMsg> {
        let reply_type = parse_reply_type(buf[0])?;

        match reply_type {
            ReplyType::Msg => {
                let mut index = 1;        
                let value_len = (&buf[index..index + 4]).get_u32() as usize;
                index += 4;
                let value = String::from_utf8(buf[index..index + value_len].to_vec()).map_err(|_| {
                    KvsError::InvalidReply
                })?;
                Ok(ReplyMsg {
                    reply_type,
                    value : Some(value),
                })
            },
            _ => {
                Ok(ReplyMsg {
                    reply_type,
                    value : None,
                })
            }
        }
    }
}