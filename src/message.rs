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

use Argument;

pub struct Message<'a> {
    pub path: &'a str,
    pub arguments: Vec<Argument<'a>>
}

impl<'a> Message<'a> {
    pub fn deserialize(buf: &'a [u8]) -> Result<Message<'a>, ()> {
        let mut msg = Message {
            path: "",
            arguments: vec![]
        };

        let mut slice = buf;

        match Argument::deserialize('s', &mut slice) {
            Ok(Argument::s(st)) => msg.path = st,
            _ => return Err(())
        }

        let typetags = match Argument::deserialize('s', &mut slice) {
            Ok(Argument::s(st)) => st,
            _ => return Err(())
        };

        for typetag in typetags[1 ..].chars() {
            let arg = Argument::deserialize(typetag, &mut slice);

            match arg {
                Ok(arg) => msg.arguments.push(arg),
                Err(_) => return Err(())
            }
        }

        Ok(msg)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        ret.extend(Argument::s(self.path).serialize());

        let mut typetags = String::from(",");

        for arg in &self.arguments {
            typetags.push(arg.typetag());
        }

        ret.extend(Argument::s(&*typetags).serialize());

        for arg in &self.arguments {
            ret.extend(arg.serialize());
        }

        ret
    }
}
