// This enum has four variants with different types:
//
// - `Quit` has no data associated with it at all.
// - `Move` includes an anonymous struct inside it.
// - `Write` includes a single String.
// - `ChangeColor` includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The following structs could hold the same data that
// the preceding enum variants hold:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, which each have
// their own type, we couldn’t as easily define a function
// to take any of these kinds of messages as we could with
// the `Message` enum.
fn main() {}
