@0xedda8f1fc8b626fe;

using Rust = import "../../../rust.capnp";

$Rust.parentModule("org::podcastindex::podping");

enum PodpingMedium {
    mixed @0;

    podcast @1;
    podcastL @2;

    music @3;
    musicL @4;

    video @5;
    videoL @6;

    film @7;
    filmL @8;

    audiobook @9;
    audiobookL @10;

    newsletter @11;
    newsletterL @12;

    blog @13;
    blogL @14;

    publisher @15;
    publisherL @16;

    course @17;
    courseL @18;
}
