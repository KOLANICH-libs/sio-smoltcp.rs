/// The error type for the networking stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ResultCode {
    /// Everything is OK, operation has succeed. Used in FFI bindings.
    OK = 0,

    /// An operation cannot proceed because a buffer is empty or full.
    Exhausted = 1,

    /// An operation is not permitted in the current state.
    Illegal = 2,

    /// An endpoint or address of a remote host could not be translated to a lower level address.
    /// E.g. there was no an Ethernet address corresponding to an IPv4 address in the ARP cache,
    /// or a TCP connection attempt was made to an unspecified endpoint.
    Unaddressable = 3,

    /// The operation is finished.
    /// E.g. when reading from a TCP socket, there's no more data to read because the remote
    /// has closed the connection.
    Finished = 4,

    /// An incoming packet could not be parsed because some of its fields were out of bounds
    /// of the received data.
    Truncated = 5,

    /// An incoming packet had an incorrect checksum and was dropped.
    Checksum = 6,

    /// An incoming packet could not be recognized and was dropped.
    /// E.g. an Ethernet packet with an unknown EtherType.
    Unrecognized = 7,

    /// An incoming IP packet has been split into several IP fragments and was dropped,
    /// since IP reassembly is not supported.
    Fragmented = 8,

    /// An incoming packet was recognized but was self-contradictory.
    /// E.g. a TCP packet with both SYN and FIN flags set.
    Malformed = 9,

    /// An incoming packet was recognized but contradicted internal state.
    /// E.g. a TCP packet addressed to a socket that doesn't exist.
    Dropped = 10,
    /// An incoming fragment arrived too late.
    ReassemblyTimeout = 11,

    /// The packet assembler is not initialized, thus it cannot know what the final size of the
    /// packet would be.
    PacketAssemblerNotInit = 12,

    /// The buffer of the assembler is to small and thus the final packet wont fit into it.
    PacketAssemblerBufferTooSmall = 13,

    /// The packet assembler did not receive all the fragments for assembling the final packet.
    PacketAssemblerIncomplete = 14,

    /// There are too many holes in the packet assembler (should be fixed in the future?).
    PacketAssemblerTooManyHoles = 15,

    /// There was an overlap when adding data to the packet assembler.
    PacketAssemblerOverlap = 16,

    /// The packet assembler set has no place for assembling a new stream of fragments.
    PacketAssemblerSetFull = 17,

    /// The key was not found in the packet assembler set.
    PacketAssemblerSetKeyNotFound = 18,

    /// An incoming packet was recognized but some parts are not supported by smoltcp.
    /// E.g. some bit configuration in a packet header is not supported, but is defined in an RFC.
    NotSupported = 19,

    InvalidState = 20,
    BufferFull = 21,
    NoFreeSlot = 22,
    InvalidName = 23,
    NameTooLong = 24,

    /// The operation is not done yet.
    Pending = 25,

    /// The operation has failed
    Failed = 26,

    // Used in FFI bindings to indicate that the provided buffer size is insufficient.
    BufferInsufficient = 0xFF,
}
