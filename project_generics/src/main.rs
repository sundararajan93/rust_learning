#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

impl ChatMessage <DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl <T> ChatMessage <T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}
/*
Let's model a real-time chat system where users can
share audio and video files.
 
Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation. --> Done
 
Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation. --> Done
 
Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile". --> Dome
 
Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct. --> Done
 
In `main`, create a ChatMessage with `content` set to a
string slice. --> Done
 
Create another ChatMessage with `content` set to a String. --> Done
 
Create another ChatMessage with `content' set to a
DigitalContent variant. --> Done
 
Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum. --> Done
 
Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

fn main() {
    let chatmessage = ChatMessage {
        content: "hey There",
        time: String::from("10.50"),
    };

    println!("{:#?}", chatmessage);

    let another_msg = ChatMessage {
        content: String::from("Hey there I'm String"),
        time: String::from("11.59")
    };
    println!("{:#?}", another_msg);

    let variabt_msg = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("12.58"),
    };
    println!("{:#?}", variabt_msg);

    variabt_msg.consume_entertainment();

    chatmessage.retrieve_time();
    another_msg.retrieve_time();
    variabt_msg.retrieve_time();
}
