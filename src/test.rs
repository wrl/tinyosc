// Copyright (c) 2015 William Light <wrl@illest.net>
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[allow(unused_imports)]
use {
    Argument,
    Message
};

macro_rules! assert_msg_equal {
    ($x:expr, $expected:expr) => {
        assert_eq!($x.unwrap(), $expected.as_bytes());
    }
}

#[test]
fn serialize_no_args() {
    let msg = Message {
        path: "/test_msg",
        arguments: osc_args![]
    };

    assert_msg_equal!(msg.serialize(), "/test_msg\0\0\0,\0\0\0");
}

#[test]
fn serialize_i32() {
    let msg = Message {
        path: "/test_msg",
        arguments: osc_args![42]
    };

    assert_msg_equal!(msg.serialize(), "/test_msg\0\0\0,i\0\0\0\0\0\x2A");
}

#[test]
fn serialize_f32() {
    let msg = Message {
        path: "/test_msg",
        arguments: osc_args![0.0 as f32]
    };

    assert_msg_equal!(msg.serialize(), "/test_msg\0\0\0,f\0\0\0\0\0\0");
}

#[test]
fn serialize_str() {
    let msg = Message {
        path: "/test_msg",
        arguments: osc_args!["testing"]
    };

    assert_msg_equal!(msg.serialize(), "/test_msg\0\0\0,s\0\0testing\0");
}

#[test]
fn serialize_kitchen_sink() {
    let msg = Message {
        path: "/test_msg",
        arguments: osc_args![42, (0.0 as f32), "testing"]
    };

    assert_msg_equal!(msg.serialize(),
        "/test_msg\0\0\0,ifs\0\0\0\0\0\0\0\x2A\0\0\0\0testing\0");
}

#[test]
fn deserialize_no_args() {
    let buf = "/test_msg\0\0\0,\0\0\0".as_bytes();
    let msg = Message::deserialize(buf).unwrap();

    assert_eq!(msg.path, "/test_msg");
    assert!(msg.arguments.len() == 0);
}

#[test]
fn deserialize_i32() {
    let buf = "/test_msg\0\0\0,i\0\0\0\0\0\x2A".as_bytes();
    let msg = Message::deserialize(buf).unwrap();

    assert_eq!(msg.path, "/test_msg");
    assert!(msg.arguments.len() == 1);

    assert!(match msg.arguments[0] {
        Argument::i(v) => (v == 42),
        _ => false
    });
}

#[test]
fn deserialize_f32() {
    let buf = "/test_msg\0\0\0,f\0\0\0\0\0\0".as_bytes();
    let msg = Message::deserialize(buf).unwrap();

    assert_eq!(msg.path, "/test_msg");
    assert!(msg.arguments.len() == 1);

    assert!(match msg.arguments[0] {
        Argument::f(v) => (v == 0.0),
        _ => false
    });
}

#[test]
fn deserialize_string() {
    let buf = "/test_msg\0\0\0,s\0\0testing\0".as_bytes();
    let msg = Message::deserialize(buf).unwrap();

    assert_eq!(msg.path, "/test_msg");
    assert!(msg.arguments.len() == 1);

    assert!(match msg.arguments[0] {
        Argument::s(v) => (v == "testing"),
        _ => false
    });
}

#[test]
fn deserialize_kitchen_sink() {
    let buf = "/test_msg\0\0\0,ifs\0\0\0\0\0\0\0\x2A\0\0\0\0testing\0".as_bytes();
    let msg = Message::deserialize(buf).unwrap();

    assert_eq!(msg.path, "/test_msg");
    assert!(msg.arguments.len() == 3);

    assert!(match msg.arguments[0] {
        Argument::i(v) => (v == 42),
        _ => false
    });

    assert!(match msg.arguments[1] {
        Argument::f(v) => (v == 0.0),
        _ => false
    });

    assert!(match msg.arguments[2] {
        Argument::s(v) => (v == "testing"),
        _ => false
    });
}
