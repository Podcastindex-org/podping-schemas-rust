@0x9b917b91f85f5cc2;

using Rust = import "../../../rust.capnp";

$Rust.parentModule("org::podcastindex::podping");

using import "../../../org/podcastindex/podping/podping_medium.capnp".PodpingMedium;
using import "../../../org/podcastindex/podping/podping_reason.capnp".PodpingReason;

struct PodpingWrite {
    medium @0 :PodpingMedium;
    reason @1 :PodpingReason;
    iri @2 :Text;
}
