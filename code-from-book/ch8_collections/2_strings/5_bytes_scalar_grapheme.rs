fn main() {
    // Another point about UTF-8 is that there are actually
    // three relevant ways to look at strings from Rust’s
    // perspective: as bytes, scalar values, and grapheme
    // clusters (the closest thing to letters).

    // Hindi word “नमस्ते” written in the Devanagari script is
    // stored as a vector of `u8` values that looks like this:
    //
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165,
    // 141, 224, 164, 164,224, 165, 135]

    // That’s 18 bytes and is how computers ultimately store
    // this data. If we look at them as Unicode scalar values,
    // which are what Rust’s `char` type is, those bytes look
    // like this:
    //
    // ['न', 'म', 'स', '्', 'त', 'े']

    // There are `six` char values here, but the fourth and sixth
    // are not letters: they’re diacritics that don’t make sense on
    // their own. Finally, if we look at them as grapheme clusters,
    // we’d get what a person would call the four letters that make
    // up the Hindi word:
    //
    // ["न", "म", "स्", "ते"]

    // A final reason Rust doesn’t allow us to index into a String
    // to get a character is that indexing operations are expected
    // to always take constant time `(O(1))`. But it isn’t possible
    // to guarantee that performance with a String, because Rust
    // would have to walk through the contents from the beginning
    // to the index to determine how many valid characters there were.
}
