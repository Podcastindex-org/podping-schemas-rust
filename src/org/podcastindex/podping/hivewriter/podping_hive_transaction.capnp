@0xb804df1ba3cc0461;

using Rust = import "rust.capnp";

$Rust.parentModule("org::podcastindex::podping::hivewriter");

using import "../podping.capnp".Podping;

struct PodpingHiveTransaction {
    podpings @0 :List(Podping);
    hiveTxId @1 :Text;
    hiveBlockNum @2 :UInt64;
}
