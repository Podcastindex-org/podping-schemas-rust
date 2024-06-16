@0x9d1d9594e023aa7c;

using Rust = import "../../../../rust.capnp";

$Rust.parentModule("org::podcastindex::podping::hivewriter");

using import "../../../../org/podcastindex/podping/podping_write.capnp".PodpingWrite;

struct PodpingHiveWrite {
    podpingWrite @0 :PodpingWrite;
    hiveAccount @1 :Text;
}
