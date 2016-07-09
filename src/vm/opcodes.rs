/// The `Opcode` type.
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Inc, // +
    IncPtr, // >
    Dec, // -
    DecPtr, // <
    Double, // *
    Halve, // /
    Print, // .
    Read, // ,
    Push, // :
    Pop, // ;
    JzCell, // [
    JnzCell, // ]
    JzStack, // (
    JnzStack, // )
    JmpStack, // ^
    Dup, // &
    Swap, // \
    Count, // #
    Compare, // =
    Str, // "
    Break, // !
    BinMod, // b
    ChrMod, // c
    IntMod, // i
    HexMod, // x
    Terminate, // q
}
