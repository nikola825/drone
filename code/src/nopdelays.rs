#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! nop1 {
    () => {{
        use core::arch::asm;
        asm!("nop");
    }};
}

pub(crate) use nop1;

macro_rules! nop2 {
    () => {{
        use core::arch::asm;
        asm!("nop");
        asm!("nop");
    }};
}

pub(crate) use nop2;

macro_rules! nop3 {
    () => {{
        use core::arch::asm;
        nop1!();
        nop1!();
        asm!("nop");
    }};
}

pub(crate) use nop3;

macro_rules! nop4 {
    () => {{
        use core::arch::asm;
        nop2!();
        nop2!();
    }};
}

pub(crate) use nop4;

macro_rules! nop5 {
    () => {{
        use core::arch::asm;
        nop2!();
        nop2!();
        asm!("nop");
    }};
}

pub(crate) use nop5;

macro_rules! nop6 {
    () => {{
        use core::arch::asm;
        nop3!();
        nop3!();
    }};
}

pub(crate) use nop6;

macro_rules! nop7 {
    () => {{
        use core::arch::asm;
        nop3!();
        nop3!();
        asm!("nop");
    }};
}

pub(crate) use nop7;

macro_rules! nop8 {
    () => {{
        use core::arch::asm;
        nop4!();
        nop4!();
    }};
}

pub(crate) use nop8;

macro_rules! nop9 {
    () => {{
        use core::arch::asm;
        nop4!();
        nop4!();
        asm!("nop");
    }};
}

pub(crate) use nop9;

macro_rules! nop10 {
    () => {{
        use core::arch::asm;
        nop5!();
        nop5!();
    }};
}

pub(crate) use nop10;

macro_rules! nop11 {
    () => {{
        use core::arch::asm;
        nop5!();
        nop5!();
        asm!("nop");
    }};
}

pub(crate) use nop11;

macro_rules! nop12 {
    () => {{
        use core::arch::asm;
        nop6!();
        nop6!();
    }};
}

pub(crate) use nop12;

macro_rules! nop13 {
    () => {{
        use core::arch::asm;
        nop6!();
        nop6!();
        asm!("nop");
    }};
}

pub(crate) use nop13;

macro_rules! nop14 {
    () => {{
        use core::arch::asm;
        nop7!();
        nop7!();
    }};
}

pub(crate) use nop14;

macro_rules! nop15 {
    () => {{
        use core::arch::asm;
        nop7!();
        nop7!();
        asm!("nop");
    }};
}

pub(crate) use nop15;

macro_rules! nop16 {
    () => {{
        use core::arch::asm;
        nop8!();
        nop8!();
    }};
}

pub(crate) use nop16;

macro_rules! nop17 {
    () => {{
        use core::arch::asm;
        nop8!();
        nop8!();
        asm!("nop");
    }};
}

pub(crate) use nop17;

macro_rules! nop18 {
    () => {{
        use core::arch::asm;
        nop9!();
        nop9!();
    }};
}

pub(crate) use nop18;

macro_rules! nop19 {
    () => {{
        use core::arch::asm;
        nop9!();
        nop9!();
        asm!("nop");
    }};
}

pub(crate) use nop19;

macro_rules! nop20 {
    () => {{
        use core::arch::asm;
        nop10!();
        nop10!();
    }};
}

pub(crate) use nop20;

macro_rules! nop21 {
    () => {{
        use core::arch::asm;
        nop10!();
        nop10!();
        asm!("nop");
    }};
}

pub(crate) use nop21;

macro_rules! nop22 {
    () => {{
        use core::arch::asm;
        nop11!();
        nop11!();
    }};
}

pub(crate) use nop22;

macro_rules! nop23 {
    () => {{
        use core::arch::asm;
        nop11!();
        nop11!();
        asm!("nop");
    }};
}

pub(crate) use nop23;

macro_rules! nop24 {
    () => {{
        use core::arch::asm;
        nop12!();
        nop12!();
    }};
}

pub(crate) use nop24;

macro_rules! nop25 {
    () => {{
        use core::arch::asm;
        nop12!();
        nop12!();
        asm!("nop");
    }};
}

pub(crate) use nop25;

macro_rules! nop26 {
    () => {{
        use core::arch::asm;
        nop13!();
        nop13!();
    }};
}

pub(crate) use nop26;

macro_rules! nop27 {
    () => {{
        use core::arch::asm;
        nop13!();
        nop13!();
        asm!("nop");
    }};
}

pub(crate) use nop27;

macro_rules! nop28 {
    () => {{
        use core::arch::asm;
        nop14!();
        nop14!();
    }};
}

pub(crate) use nop28;

macro_rules! nop29 {
    () => {{
        use core::arch::asm;
        nop14!();
        nop14!();
        asm!("nop");
    }};
}

pub(crate) use nop29;

macro_rules! nop30 {
    () => {{
        use core::arch::asm;
        nop15!();
        nop15!();
    }};
}

pub(crate) use nop30;

macro_rules! nop31 {
    () => {{
        use core::arch::asm;
        nop15!();
        nop15!();
        asm!("nop");
    }};
}

pub(crate) use nop31;

macro_rules! nop32 {
    () => {{
        use core::arch::asm;
        nop16!();
        nop16!();
    }};
}

pub(crate) use nop32;

macro_rules! nop33 {
    () => {{
        use core::arch::asm;
        nop16!();
        nop16!();
        asm!("nop");
    }};
}

pub(crate) use nop33;

macro_rules! nop34 {
    () => {{
        use core::arch::asm;
        nop17!();
        nop17!();
    }};
}

pub(crate) use nop34;

macro_rules! nop35 {
    () => {{
        use core::arch::asm;
        nop17!();
        nop17!();
        asm!("nop");
    }};
}

pub(crate) use nop35;

macro_rules! nop36 {
    () => {{
        use core::arch::asm;
        nop18!();
        nop18!();
    }};
}

pub(crate) use nop36;

macro_rules! nop37 {
    () => {{
        use core::arch::asm;
        nop18!();
        nop18!();
        asm!("nop");
    }};
}

pub(crate) use nop37;

macro_rules! nop38 {
    () => {{
        use core::arch::asm;
        nop19!();
        nop19!();
    }};
}

pub(crate) use nop38;

macro_rules! nop39 {
    () => {{
        use core::arch::asm;
        nop19!();
        nop19!();
        asm!("nop");
    }};
}

pub(crate) use nop39;

macro_rules! nop40 {
    () => {{
        use core::arch::asm;
        nop20!();
        nop20!();
    }};
}

pub(crate) use nop40;

macro_rules! nop41 {
    () => {{
        use core::arch::asm;
        nop20!();
        nop20!();
        asm!("nop");
    }};
}

pub(crate) use nop41;

macro_rules! nop42 {
    () => {{
        use core::arch::asm;
        nop21!();
        nop21!();
    }};
}

pub(crate) use nop42;

macro_rules! nop43 {
    () => {{
        use core::arch::asm;
        nop21!();
        nop21!();
        asm!("nop");
    }};
}

pub(crate) use nop43;

macro_rules! nop44 {
    () => {{
        use core::arch::asm;
        nop22!();
        nop22!();
    }};
}

pub(crate) use nop44;

macro_rules! nop45 {
    () => {{
        use core::arch::asm;
        nop22!();
        nop22!();
        asm!("nop");
    }};
}

pub(crate) use nop45;

macro_rules! nop46 {
    () => {{
        use core::arch::asm;
        nop23!();
        nop23!();
    }};
}

pub(crate) use nop46;

macro_rules! nop47 {
    () => {{
        use core::arch::asm;
        nop23!();
        nop23!();
        asm!("nop");
    }};
}

pub(crate) use nop47;

macro_rules! nop48 {
    () => {{
        use core::arch::asm;
        nop24!();
        nop24!();
    }};
}

pub(crate) use nop48;

macro_rules! nop49 {
    () => {{
        use core::arch::asm;
        nop24!();
        nop24!();
        asm!("nop");
    }};
}

pub(crate) use nop49;

macro_rules! nop50 {
    () => {{
        use core::arch::asm;
        nop25!();
        nop25!();
    }};
}

pub(crate) use nop50;

macro_rules! nop51 {
    () => {{
        use core::arch::asm;
        nop25!();
        nop25!();
        asm!("nop");
    }};
}

pub(crate) use nop51;

macro_rules! nop52 {
    () => {{
        use core::arch::asm;
        nop26!();
        nop26!();
    }};
}

pub(crate) use nop52;

macro_rules! nop53 {
    () => {{
        use core::arch::asm;
        nop26!();
        nop26!();
        asm!("nop");
    }};
}

pub(crate) use nop53;

macro_rules! nop54 {
    () => {{
        use core::arch::asm;
        nop27!();
        nop27!();
    }};
}

pub(crate) use nop54;

macro_rules! nop55 {
    () => {{
        use core::arch::asm;
        nop27!();
        nop27!();
        asm!("nop");
    }};
}

pub(crate) use nop55;

macro_rules! nop56 {
    () => {{
        use core::arch::asm;
        nop28!();
        nop28!();
    }};
}

pub(crate) use nop56;

macro_rules! nop57 {
    () => {{
        use core::arch::asm;
        nop28!();
        nop28!();
        asm!("nop");
    }};
}

pub(crate) use nop57;

macro_rules! nop58 {
    () => {{
        use core::arch::asm;
        nop29!();
        nop29!();
    }};
}

pub(crate) use nop58;

macro_rules! nop59 {
    () => {{
        use core::arch::asm;
        nop29!();
        nop29!();
        asm!("nop");
    }};
}

pub(crate) use nop59;

macro_rules! nop60 {
    () => {{
        use core::arch::asm;
        nop30!();
        nop30!();
    }};
}

pub(crate) use nop60;

macro_rules! nop61 {
    () => {{
        use core::arch::asm;
        nop30!();
        nop30!();
        asm!("nop");
    }};
}

pub(crate) use nop61;

macro_rules! nop62 {
    () => {{
        use core::arch::asm;
        nop31!();
        nop31!();
    }};
}

pub(crate) use nop62;

macro_rules! nop63 {
    () => {{
        use core::arch::asm;
        nop31!();
        nop31!();
        asm!("nop");
    }};
}

pub(crate) use nop63;

macro_rules! nop64 {
    () => {{
        use core::arch::asm;
        nop32!();
        nop32!();
    }};
}

pub(crate) use nop64;

macro_rules! nop65 {
    () => {{
        use core::arch::asm;
        nop32!();
        nop32!();
        asm!("nop");
    }};
}

pub(crate) use nop65;

macro_rules! nop66 {
    () => {{
        use core::arch::asm;
        nop33!();
        nop33!();
    }};
}

pub(crate) use nop66;

macro_rules! nop67 {
    () => {{
        use core::arch::asm;
        nop33!();
        nop33!();
        asm!("nop");
    }};
}

pub(crate) use nop67;

macro_rules! nop68 {
    () => {{
        use core::arch::asm;
        nop34!();
        nop34!();
    }};
}

pub(crate) use nop68;

macro_rules! nop69 {
    () => {{
        use core::arch::asm;
        nop34!();
        nop34!();
        asm!("nop");
    }};
}

pub(crate) use nop69;

macro_rules! nop70 {
    () => {{
        use core::arch::asm;
        nop35!();
        nop35!();
    }};
}

pub(crate) use nop70;

macro_rules! nop71 {
    () => {{
        use core::arch::asm;
        nop35!();
        nop35!();
        asm!("nop");
    }};
}

pub(crate) use nop71;

macro_rules! nop72 {
    () => {{
        use core::arch::asm;
        nop36!();
        nop36!();
    }};
}

pub(crate) use nop72;

macro_rules! nop73 {
    () => {{
        use core::arch::asm;
        nop36!();
        nop36!();
        asm!("nop");
    }};
}

pub(crate) use nop73;

macro_rules! nop74 {
    () => {{
        use core::arch::asm;
        nop37!();
        nop37!();
    }};
}

pub(crate) use nop74;

macro_rules! nop75 {
    () => {{
        use core::arch::asm;
        nop37!();
        nop37!();
        asm!("nop");
    }};
}

pub(crate) use nop75;

macro_rules! nop76 {
    () => {{
        use core::arch::asm;
        nop38!();
        nop38!();
    }};
}

pub(crate) use nop76;

macro_rules! nop77 {
    () => {{
        use core::arch::asm;
        nop38!();
        nop38!();
        asm!("nop");
    }};
}

pub(crate) use nop77;

macro_rules! nop78 {
    () => {{
        use core::arch::asm;
        nop39!();
        nop39!();
    }};
}

pub(crate) use nop78;

macro_rules! nop79 {
    () => {{
        use core::arch::asm;
        nop39!();
        nop39!();
        asm!("nop");
    }};
}

pub(crate) use nop79;

macro_rules! nop80 {
    () => {{
        use core::arch::asm;
        nop40!();
        nop40!();
    }};
}

pub(crate) use nop80;

macro_rules! nop81 {
    () => {{
        use core::arch::asm;
        nop40!();
        nop40!();
        asm!("nop");
    }};
}

pub(crate) use nop81;

macro_rules! nop82 {
    () => {{
        use core::arch::asm;
        nop41!();
        nop41!();
    }};
}

pub(crate) use nop82;

macro_rules! nop83 {
    () => {{
        use core::arch::asm;
        nop41!();
        nop41!();
        asm!("nop");
    }};
}

pub(crate) use nop83;

macro_rules! nop84 {
    () => {{
        use core::arch::asm;
        nop42!();
        nop42!();
    }};
}

pub(crate) use nop84;

macro_rules! nop85 {
    () => {{
        use core::arch::asm;
        nop42!();
        nop42!();
        asm!("nop");
    }};
}

pub(crate) use nop85;

macro_rules! nop86 {
    () => {{
        use core::arch::asm;
        nop43!();
        nop43!();
    }};
}

pub(crate) use nop86;

macro_rules! nop87 {
    () => {{
        use core::arch::asm;
        nop43!();
        nop43!();
        asm!("nop");
    }};
}

pub(crate) use nop87;

macro_rules! nop88 {
    () => {{
        use core::arch::asm;
        nop44!();
        nop44!();
    }};
}

pub(crate) use nop88;

macro_rules! nop89 {
    () => {{
        use core::arch::asm;
        nop44!();
        nop44!();
        asm!("nop");
    }};
}

pub(crate) use nop89;

macro_rules! nop90 {
    () => {{
        use core::arch::asm;
        nop45!();
        nop45!();
    }};
}

pub(crate) use nop90;

macro_rules! nop91 {
    () => {{
        use core::arch::asm;
        nop45!();
        nop45!();
        asm!("nop");
    }};
}

pub(crate) use nop91;

macro_rules! nop92 {
    () => {{
        use core::arch::asm;
        nop46!();
        nop46!();
    }};
}

pub(crate) use nop92;

macro_rules! nop93 {
    () => {{
        use core::arch::asm;
        nop46!();
        nop46!();
        asm!("nop");
    }};
}

pub(crate) use nop93;

macro_rules! nop94 {
    () => {{
        use core::arch::asm;
        nop47!();
        nop47!();
    }};
}

pub(crate) use nop94;

macro_rules! nop95 {
    () => {{
        use core::arch::asm;
        nop47!();
        nop47!();
        asm!("nop");
    }};
}

pub(crate) use nop95;

macro_rules! nop96 {
    () => {{
        use core::arch::asm;
        nop48!();
        nop48!();
    }};
}

pub(crate) use nop96;

macro_rules! nop97 {
    () => {{
        use core::arch::asm;
        nop48!();
        nop48!();
        asm!("nop");
    }};
}

pub(crate) use nop97;

macro_rules! nop98 {
    () => {{
        use core::arch::asm;
        nop49!();
        nop49!();
    }};
}

pub(crate) use nop98;

macro_rules! nop99 {
    () => {{
        use core::arch::asm;
        nop49!();
        nop49!();
        asm!("nop");
    }};
}

pub(crate) use nop99;

macro_rules! nop100 {
    () => {{
        use core::arch::asm;
        nop50!();
        nop50!();
    }};
}

pub(crate) use nop100;

macro_rules! nop101 {
    () => {{
        use core::arch::asm;
        nop50!();
        nop50!();
        asm!("nop");
    }};
}

pub(crate) use nop101;

macro_rules! nop102 {
    () => {{
        use core::arch::asm;
        nop51!();
        nop51!();
    }};
}

pub(crate) use nop102;

macro_rules! nop103 {
    () => {{
        use core::arch::asm;
        nop51!();
        nop51!();
        asm!("nop");
    }};
}

pub(crate) use nop103;

macro_rules! nop104 {
    () => {{
        use core::arch::asm;
        nop52!();
        nop52!();
    }};
}

pub(crate) use nop104;

macro_rules! nop105 {
    () => {{
        use core::arch::asm;
        nop52!();
        nop52!();
        asm!("nop");
    }};
}

pub(crate) use nop105;

macro_rules! nop106 {
    () => {{
        use core::arch::asm;
        nop53!();
        nop53!();
    }};
}

pub(crate) use nop106;

macro_rules! nop107 {
    () => {{
        use core::arch::asm;
        nop53!();
        nop53!();
        asm!("nop");
    }};
}

pub(crate) use nop107;

macro_rules! nop108 {
    () => {{
        use core::arch::asm;
        nop54!();
        nop54!();
    }};
}

pub(crate) use nop108;

macro_rules! nop109 {
    () => {{
        use core::arch::asm;
        nop54!();
        nop54!();
        asm!("nop");
    }};
}

pub(crate) use nop109;

macro_rules! nop110 {
    () => {{
        use core::arch::asm;
        nop55!();
        nop55!();
    }};
}

pub(crate) use nop110;

macro_rules! nop111 {
    () => {{
        use core::arch::asm;
        nop55!();
        nop55!();
        asm!("nop");
    }};
}

pub(crate) use nop111;

macro_rules! nop112 {
    () => {{
        use core::arch::asm;
        nop56!();
        nop56!();
    }};
}

pub(crate) use nop112;

macro_rules! nop113 {
    () => {{
        use core::arch::asm;
        nop56!();
        nop56!();
        asm!("nop");
    }};
}

pub(crate) use nop113;

macro_rules! nop114 {
    () => {{
        use core::arch::asm;
        nop57!();
        nop57!();
    }};
}

pub(crate) use nop114;

macro_rules! nop115 {
    () => {{
        use core::arch::asm;
        nop57!();
        nop57!();
        asm!("nop");
    }};
}

pub(crate) use nop115;

macro_rules! nop116 {
    () => {{
        use core::arch::asm;
        nop58!();
        nop58!();
    }};
}

pub(crate) use nop116;

macro_rules! nop117 {
    () => {{
        use core::arch::asm;
        nop58!();
        nop58!();
        asm!("nop");
    }};
}

pub(crate) use nop117;

macro_rules! nop118 {
    () => {{
        use core::arch::asm;
        nop59!();
        nop59!();
    }};
}

pub(crate) use nop118;

macro_rules! nop119 {
    () => {{
        use core::arch::asm;
        nop59!();
        nop59!();
        asm!("nop");
    }};
}

pub(crate) use nop119;

macro_rules! nop120 {
    () => {{
        use core::arch::asm;
        nop60!();
        nop60!();
    }};
}

pub(crate) use nop120;

macro_rules! nop121 {
    () => {{
        use core::arch::asm;
        nop60!();
        nop60!();
        asm!("nop");
    }};
}

pub(crate) use nop121;

macro_rules! nop122 {
    () => {{
        use core::arch::asm;
        nop61!();
        nop61!();
    }};
}

pub(crate) use nop122;

macro_rules! nop123 {
    () => {{
        use core::arch::asm;
        nop61!();
        nop61!();
        asm!("nop");
    }};
}

pub(crate) use nop123;

macro_rules! nop124 {
    () => {{
        use core::arch::asm;
        nop62!();
        nop62!();
    }};
}

pub(crate) use nop124;

macro_rules! nop125 {
    () => {{
        use core::arch::asm;
        nop62!();
        nop62!();
        asm!("nop");
    }};
}

pub(crate) use nop125;

macro_rules! nop126 {
    () => {{
        use core::arch::asm;
        nop63!();
        nop63!();
    }};
}

pub(crate) use nop126;

macro_rules! nop127 {
    () => {{
        use core::arch::asm;
        nop63!();
        nop63!();
        asm!("nop");
    }};
}

pub(crate) use nop127;

macro_rules! nop128 {
    () => {{
        use core::arch::asm;
        nop64!();
        nop64!();
    }};
}

pub(crate) use nop128;

macro_rules! nop129 {
    () => {{
        use core::arch::asm;
        nop64!();
        nop64!();
        asm!("nop");
    }};
}

pub(crate) use nop129;

macro_rules! nop130 {
    () => {{
        use core::arch::asm;
        nop65!();
        nop65!();
    }};
}

pub(crate) use nop130;

macro_rules! nop131 {
    () => {{
        use core::arch::asm;
        nop65!();
        nop65!();
        asm!("nop");
    }};
}

pub(crate) use nop131;

macro_rules! nop132 {
    () => {{
        use core::arch::asm;
        nop66!();
        nop66!();
    }};
}

pub(crate) use nop132;

macro_rules! nop133 {
    () => {{
        use core::arch::asm;
        nop66!();
        nop66!();
        asm!("nop");
    }};
}

pub(crate) use nop133;

macro_rules! nop134 {
    () => {{
        use core::arch::asm;
        nop67!();
        nop67!();
    }};
}

pub(crate) use nop134;

macro_rules! nop135 {
    () => {{
        use core::arch::asm;
        nop67!();
        nop67!();
        asm!("nop");
    }};
}

pub(crate) use nop135;

macro_rules! nop136 {
    () => {{
        use core::arch::asm;
        nop68!();
        nop68!();
    }};
}

pub(crate) use nop136;

macro_rules! nop137 {
    () => {{
        use core::arch::asm;
        nop68!();
        nop68!();
        asm!("nop");
    }};
}

pub(crate) use nop137;

macro_rules! nop138 {
    () => {{
        use core::arch::asm;
        nop69!();
        nop69!();
    }};
}

pub(crate) use nop138;

macro_rules! nop139 {
    () => {{
        use core::arch::asm;
        nop69!();
        nop69!();
        asm!("nop");
    }};
}

pub(crate) use nop139;

macro_rules! nop140 {
    () => {{
        use core::arch::asm;
        nop70!();
        nop70!();
    }};
}

pub(crate) use nop140;

macro_rules! nop141 {
    () => {{
        use core::arch::asm;
        nop70!();
        nop70!();
        asm!("nop");
    }};
}

pub(crate) use nop141;

macro_rules! nop142 {
    () => {{
        use core::arch::asm;
        nop71!();
        nop71!();
    }};
}

pub(crate) use nop142;

macro_rules! nop143 {
    () => {{
        use core::arch::asm;
        nop71!();
        nop71!();
        asm!("nop");
    }};
}

pub(crate) use nop143;

macro_rules! nop144 {
    () => {{
        use core::arch::asm;
        nop72!();
        nop72!();
    }};
}

pub(crate) use nop144;

macro_rules! nop145 {
    () => {{
        use core::arch::asm;
        nop72!();
        nop72!();
        asm!("nop");
    }};
}

pub(crate) use nop145;

macro_rules! nop146 {
    () => {{
        use core::arch::asm;
        nop73!();
        nop73!();
    }};
}

pub(crate) use nop146;

macro_rules! nop147 {
    () => {{
        use core::arch::asm;
        nop73!();
        nop73!();
        asm!("nop");
    }};
}

pub(crate) use nop147;

macro_rules! nop148 {
    () => {{
        use core::arch::asm;
        nop74!();
        nop74!();
    }};
}

pub(crate) use nop148;

macro_rules! nop149 {
    () => {{
        use core::arch::asm;
        nop74!();
        nop74!();
        asm!("nop");
    }};
}

pub(crate) use nop149;

macro_rules! nop150 {
    () => {{
        use core::arch::asm;
        nop75!();
        nop75!();
    }};
}

pub(crate) use nop150;

macro_rules! nop151 {
    () => {{
        use core::arch::asm;
        nop75!();
        nop75!();
        asm!("nop");
    }};
}

pub(crate) use nop151;

macro_rules! nop152 {
    () => {{
        use core::arch::asm;
        nop76!();
        nop76!();
    }};
}

pub(crate) use nop152;

macro_rules! nop153 {
    () => {{
        use core::arch::asm;
        nop76!();
        nop76!();
        asm!("nop");
    }};
}

pub(crate) use nop153;

macro_rules! nop154 {
    () => {{
        use core::arch::asm;
        nop77!();
        nop77!();
    }};
}

pub(crate) use nop154;

macro_rules! nop155 {
    () => {{
        use core::arch::asm;
        nop77!();
        nop77!();
        asm!("nop");
    }};
}

pub(crate) use nop155;

macro_rules! nop156 {
    () => {{
        use core::arch::asm;
        nop78!();
        nop78!();
    }};
}

pub(crate) use nop156;

macro_rules! nop157 {
    () => {{
        use core::arch::asm;
        nop78!();
        nop78!();
        asm!("nop");
    }};
}

pub(crate) use nop157;

macro_rules! nop158 {
    () => {{
        use core::arch::asm;
        nop79!();
        nop79!();
    }};
}

pub(crate) use nop158;

macro_rules! nop159 {
    () => {{
        use core::arch::asm;
        nop79!();
        nop79!();
        asm!("nop");
    }};
}

pub(crate) use nop159;

macro_rules! nop160 {
    () => {{
        use core::arch::asm;
        nop80!();
        nop80!();
    }};
}

pub(crate) use nop160;

macro_rules! nop161 {
    () => {{
        use core::arch::asm;
        nop80!();
        nop80!();
        asm!("nop");
    }};
}

pub(crate) use nop161;

macro_rules! nop162 {
    () => {{
        use core::arch::asm;
        nop81!();
        nop81!();
    }};
}

pub(crate) use nop162;

macro_rules! nop163 {
    () => {{
        use core::arch::asm;
        nop81!();
        nop81!();
        asm!("nop");
    }};
}

pub(crate) use nop163;

macro_rules! nop164 {
    () => {{
        use core::arch::asm;
        nop82!();
        nop82!();
    }};
}

pub(crate) use nop164;

macro_rules! nop165 {
    () => {{
        use core::arch::asm;
        nop82!();
        nop82!();
        asm!("nop");
    }};
}

pub(crate) use nop165;

macro_rules! nop166 {
    () => {{
        use core::arch::asm;
        nop83!();
        nop83!();
    }};
}

pub(crate) use nop166;

macro_rules! nop167 {
    () => {{
        use core::arch::asm;
        nop83!();
        nop83!();
        asm!("nop");
    }};
}

pub(crate) use nop167;

macro_rules! nop168 {
    () => {{
        use core::arch::asm;
        nop84!();
        nop84!();
    }};
}

pub(crate) use nop168;

macro_rules! nop169 {
    () => {{
        use core::arch::asm;
        nop84!();
        nop84!();
        asm!("nop");
    }};
}

pub(crate) use nop169;

macro_rules! nop170 {
    () => {{
        use core::arch::asm;
        nop85!();
        nop85!();
    }};
}

pub(crate) use nop170;

macro_rules! nop171 {
    () => {{
        use core::arch::asm;
        nop85!();
        nop85!();
        asm!("nop");
    }};
}

pub(crate) use nop171;

macro_rules! nop172 {
    () => {{
        use core::arch::asm;
        nop86!();
        nop86!();
    }};
}

pub(crate) use nop172;

macro_rules! nop173 {
    () => {{
        use core::arch::asm;
        nop86!();
        nop86!();
        asm!("nop");
    }};
}

pub(crate) use nop173;

macro_rules! nop174 {
    () => {{
        use core::arch::asm;
        nop87!();
        nop87!();
    }};
}

pub(crate) use nop174;

macro_rules! nop175 {
    () => {{
        use core::arch::asm;
        nop87!();
        nop87!();
        asm!("nop");
    }};
}

pub(crate) use nop175;

macro_rules! nop176 {
    () => {{
        use core::arch::asm;
        nop88!();
        nop88!();
    }};
}

pub(crate) use nop176;

macro_rules! nop177 {
    () => {{
        use core::arch::asm;
        nop88!();
        nop88!();
        asm!("nop");
    }};
}

pub(crate) use nop177;

macro_rules! nop178 {
    () => {{
        use core::arch::asm;
        nop89!();
        nop89!();
    }};
}

pub(crate) use nop178;

macro_rules! nop179 {
    () => {{
        use core::arch::asm;
        nop89!();
        nop89!();
        asm!("nop");
    }};
}

pub(crate) use nop179;

macro_rules! nop180 {
    () => {{
        use core::arch::asm;
        nop90!();
        nop90!();
    }};
}

pub(crate) use nop180;

macro_rules! nop181 {
    () => {{
        use core::arch::asm;
        nop90!();
        nop90!();
        asm!("nop");
    }};
}

pub(crate) use nop181;

macro_rules! nop182 {
    () => {{
        use core::arch::asm;
        nop91!();
        nop91!();
    }};
}

pub(crate) use nop182;

macro_rules! nop183 {
    () => {{
        use core::arch::asm;
        nop91!();
        nop91!();
        asm!("nop");
    }};
}

pub(crate) use nop183;

macro_rules! nop184 {
    () => {{
        use core::arch::asm;
        nop92!();
        nop92!();
    }};
}

pub(crate) use nop184;

macro_rules! nop185 {
    () => {{
        use core::arch::asm;
        nop92!();
        nop92!();
        asm!("nop");
    }};
}

pub(crate) use nop185;

macro_rules! nop186 {
    () => {{
        use core::arch::asm;
        nop93!();
        nop93!();
    }};
}

pub(crate) use nop186;

macro_rules! nop187 {
    () => {{
        use core::arch::asm;
        nop93!();
        nop93!();
        asm!("nop");
    }};
}

pub(crate) use nop187;

macro_rules! nop188 {
    () => {{
        use core::arch::asm;
        nop94!();
        nop94!();
    }};
}

pub(crate) use nop188;

macro_rules! nop189 {
    () => {{
        use core::arch::asm;
        nop94!();
        nop94!();
        asm!("nop");
    }};
}

pub(crate) use nop189;

macro_rules! nop190 {
    () => {{
        use core::arch::asm;
        nop95!();
        nop95!();
    }};
}

pub(crate) use nop190;

macro_rules! nop191 {
    () => {{
        use core::arch::asm;
        nop95!();
        nop95!();
        asm!("nop");
    }};
}

pub(crate) use nop191;

macro_rules! nop192 {
    () => {{
        use core::arch::asm;
        nop96!();
        nop96!();
    }};
}

pub(crate) use nop192;

macro_rules! nop193 {
    () => {{
        use core::arch::asm;
        nop96!();
        nop96!();
        asm!("nop");
    }};
}

pub(crate) use nop193;

macro_rules! nop194 {
    () => {{
        use core::arch::asm;
        nop97!();
        nop97!();
    }};
}

pub(crate) use nop194;

macro_rules! nop195 {
    () => {{
        use core::arch::asm;
        nop97!();
        nop97!();
        asm!("nop");
    }};
}

pub(crate) use nop195;

macro_rules! nop196 {
    () => {{
        use core::arch::asm;
        nop98!();
        nop98!();
    }};
}

pub(crate) use nop196;

macro_rules! nop197 {
    () => {{
        use core::arch::asm;
        nop98!();
        nop98!();
        asm!("nop");
    }};
}

pub(crate) use nop197;

macro_rules! nop198 {
    () => {{
        use core::arch::asm;
        nop99!();
        nop99!();
    }};
}

pub(crate) use nop198;

macro_rules! nop199 {
    () => {{
        use core::arch::asm;
        nop99!();
        nop99!();
        asm!("nop");
    }};
}

pub(crate) use nop199;

macro_rules! nop200 {
    () => {{
        use core::arch::asm;
        nop100!();
        nop100!();
    }};
}

pub(crate) use nop200;

macro_rules! nop201 {
    () => {{
        use core::arch::asm;
        nop100!();
        nop100!();
        asm!("nop");
    }};
}

pub(crate) use nop201;

macro_rules! nop202 {
    () => {{
        use core::arch::asm;
        nop101!();
        nop101!();
    }};
}

pub(crate) use nop202;

macro_rules! nop203 {
    () => {{
        use core::arch::asm;
        nop101!();
        nop101!();
        asm!("nop");
    }};
}

pub(crate) use nop203;

macro_rules! nop204 {
    () => {{
        use core::arch::asm;
        nop102!();
        nop102!();
    }};
}

pub(crate) use nop204;

macro_rules! nop205 {
    () => {{
        use core::arch::asm;
        nop102!();
        nop102!();
        asm!("nop");
    }};
}

pub(crate) use nop205;

macro_rules! nop206 {
    () => {{
        use core::arch::asm;
        nop103!();
        nop103!();
    }};
}

pub(crate) use nop206;

macro_rules! nop207 {
    () => {{
        use core::arch::asm;
        nop103!();
        nop103!();
        asm!("nop");
    }};
}

pub(crate) use nop207;

macro_rules! nop208 {
    () => {{
        use core::arch::asm;
        nop104!();
        nop104!();
    }};
}

pub(crate) use nop208;

macro_rules! nop209 {
    () => {{
        use core::arch::asm;
        nop104!();
        nop104!();
        asm!("nop");
    }};
}

pub(crate) use nop209;

macro_rules! nop210 {
    () => {{
        use core::arch::asm;
        nop105!();
        nop105!();
    }};
}

pub(crate) use nop210;

macro_rules! nop211 {
    () => {{
        use core::arch::asm;
        nop105!();
        nop105!();
        asm!("nop");
    }};
}

pub(crate) use nop211;

macro_rules! nop212 {
    () => {{
        use core::arch::asm;
        nop106!();
        nop106!();
    }};
}

pub(crate) use nop212;

macro_rules! nop213 {
    () => {{
        use core::arch::asm;
        nop106!();
        nop106!();
        asm!("nop");
    }};
}

pub(crate) use nop213;

macro_rules! nop214 {
    () => {{
        use core::arch::asm;
        nop107!();
        nop107!();
    }};
}

pub(crate) use nop214;

macro_rules! nop215 {
    () => {{
        use core::arch::asm;
        nop107!();
        nop107!();
        asm!("nop");
    }};
}

pub(crate) use nop215;

macro_rules! nop216 {
    () => {{
        use core::arch::asm;
        nop108!();
        nop108!();
    }};
}

pub(crate) use nop216;

macro_rules! nop217 {
    () => {{
        use core::arch::asm;
        nop108!();
        nop108!();
        asm!("nop");
    }};
}

pub(crate) use nop217;

macro_rules! nop218 {
    () => {{
        use core::arch::asm;
        nop109!();
        nop109!();
    }};
}

pub(crate) use nop218;

macro_rules! nop219 {
    () => {{
        use core::arch::asm;
        nop109!();
        nop109!();
        asm!("nop");
    }};
}

pub(crate) use nop219;

macro_rules! nop220 {
    () => {{
        use core::arch::asm;
        nop110!();
        nop110!();
    }};
}

pub(crate) use nop220;

macro_rules! nop221 {
    () => {{
        use core::arch::asm;
        nop110!();
        nop110!();
        asm!("nop");
    }};
}

pub(crate) use nop221;

macro_rules! nop222 {
    () => {{
        use core::arch::asm;
        nop111!();
        nop111!();
    }};
}

pub(crate) use nop222;

macro_rules! nop223 {
    () => {{
        use core::arch::asm;
        nop111!();
        nop111!();
        asm!("nop");
    }};
}

pub(crate) use nop223;

macro_rules! nop224 {
    () => {{
        use core::arch::asm;
        nop112!();
        nop112!();
    }};
}

pub(crate) use nop224;

macro_rules! nop225 {
    () => {{
        use core::arch::asm;
        nop112!();
        nop112!();
        asm!("nop");
    }};
}

pub(crate) use nop225;

macro_rules! nop226 {
    () => {{
        use core::arch::asm;
        nop113!();
        nop113!();
    }};
}

pub(crate) use nop226;

macro_rules! nop227 {
    () => {{
        use core::arch::asm;
        nop113!();
        nop113!();
        asm!("nop");
    }};
}

pub(crate) use nop227;

macro_rules! nop228 {
    () => {{
        use core::arch::asm;
        nop114!();
        nop114!();
    }};
}

pub(crate) use nop228;

macro_rules! nop229 {
    () => {{
        use core::arch::asm;
        nop114!();
        nop114!();
        asm!("nop");
    }};
}

pub(crate) use nop229;

macro_rules! nop230 {
    () => {{
        use core::arch::asm;
        nop115!();
        nop115!();
    }};
}

pub(crate) use nop230;

macro_rules! nop231 {
    () => {{
        use core::arch::asm;
        nop115!();
        nop115!();
        asm!("nop");
    }};
}

pub(crate) use nop231;

macro_rules! nop232 {
    () => {{
        use core::arch::asm;
        nop116!();
        nop116!();
    }};
}

pub(crate) use nop232;

macro_rules! nop233 {
    () => {{
        use core::arch::asm;
        nop116!();
        nop116!();
        asm!("nop");
    }};
}

pub(crate) use nop233;

macro_rules! nop234 {
    () => {{
        use core::arch::asm;
        nop117!();
        nop117!();
    }};
}

pub(crate) use nop234;

macro_rules! nop235 {
    () => {{
        use core::arch::asm;
        nop117!();
        nop117!();
        asm!("nop");
    }};
}

pub(crate) use nop235;

macro_rules! nop236 {
    () => {{
        use core::arch::asm;
        nop118!();
        nop118!();
    }};
}

pub(crate) use nop236;

macro_rules! nop237 {
    () => {{
        use core::arch::asm;
        nop118!();
        nop118!();
        asm!("nop");
    }};
}

pub(crate) use nop237;

macro_rules! nop238 {
    () => {{
        use core::arch::asm;
        nop119!();
        nop119!();
    }};
}

pub(crate) use nop238;

macro_rules! nop239 {
    () => {{
        use core::arch::asm;
        nop119!();
        nop119!();
        asm!("nop");
    }};
}

pub(crate) use nop239;

macro_rules! nop240 {
    () => {{
        use core::arch::asm;
        nop120!();
        nop120!();
    }};
}

pub(crate) use nop240;

macro_rules! nop241 {
    () => {{
        use core::arch::asm;
        nop120!();
        nop120!();
        asm!("nop");
    }};
}

pub(crate) use nop241;

macro_rules! nop242 {
    () => {{
        use core::arch::asm;
        nop121!();
        nop121!();
    }};
}

pub(crate) use nop242;

macro_rules! nop243 {
    () => {{
        use core::arch::asm;
        nop121!();
        nop121!();
        asm!("nop");
    }};
}

pub(crate) use nop243;

macro_rules! nop244 {
    () => {{
        use core::arch::asm;
        nop122!();
        nop122!();
    }};
}

pub(crate) use nop244;

macro_rules! nop245 {
    () => {{
        use core::arch::asm;
        nop122!();
        nop122!();
        asm!("nop");
    }};
}

pub(crate) use nop245;

macro_rules! nop246 {
    () => {{
        use core::arch::asm;
        nop123!();
        nop123!();
    }};
}

pub(crate) use nop246;

macro_rules! nop247 {
    () => {{
        use core::arch::asm;
        nop123!();
        nop123!();
        asm!("nop");
    }};
}

pub(crate) use nop247;

macro_rules! nop248 {
    () => {{
        use core::arch::asm;
        nop124!();
        nop124!();
    }};
}

pub(crate) use nop248;

macro_rules! nop249 {
    () => {{
        use core::arch::asm;
        nop124!();
        nop124!();
        asm!("nop");
    }};
}

pub(crate) use nop249;

macro_rules! nop250 {
    () => {{
        use core::arch::asm;
        nop125!();
        nop125!();
    }};
}

pub(crate) use nop250;

macro_rules! nop251 {
    () => {{
        use core::arch::asm;
        nop125!();
        nop125!();
        asm!("nop");
    }};
}

pub(crate) use nop251;

macro_rules! nop252 {
    () => {{
        use core::arch::asm;
        nop126!();
        nop126!();
    }};
}

pub(crate) use nop252;

macro_rules! nop253 {
    () => {{
        use core::arch::asm;
        nop126!();
        nop126!();
        asm!("nop");
    }};
}

pub(crate) use nop253;

macro_rules! nop254 {
    () => {{
        use core::arch::asm;
        nop127!();
        nop127!();
    }};
}

pub(crate) use nop254;

macro_rules! nop255 {
    () => {{
        use core::arch::asm;
        nop127!();
        nop127!();
        asm!("nop");
    }};
}

pub(crate) use nop255;

macro_rules! nop256 {
    () => {{
        use core::arch::asm;
        nop128!();
        nop128!();
    }};
}

pub(crate) use nop256;

macro_rules! nop257 {
    () => {{
        use core::arch::asm;
        nop128!();
        nop128!();
        asm!("nop");
    }};
}

pub(crate) use nop257;

macro_rules! nop258 {
    () => {{
        use core::arch::asm;
        nop129!();
        nop129!();
    }};
}

pub(crate) use nop258;

macro_rules! nop259 {
    () => {{
        use core::arch::asm;
        nop129!();
        nop129!();
        asm!("nop");
    }};
}

pub(crate) use nop259;

macro_rules! nop260 {
    () => {{
        use core::arch::asm;
        nop130!();
        nop130!();
    }};
}

pub(crate) use nop260;

macro_rules! nop261 {
    () => {{
        use core::arch::asm;
        nop130!();
        nop130!();
        asm!("nop");
    }};
}

pub(crate) use nop261;

macro_rules! nop262 {
    () => {{
        use core::arch::asm;
        nop131!();
        nop131!();
    }};
}

pub(crate) use nop262;

macro_rules! nop263 {
    () => {{
        use core::arch::asm;
        nop131!();
        nop131!();
        asm!("nop");
    }};
}

pub(crate) use nop263;

macro_rules! nop264 {
    () => {{
        use core::arch::asm;
        nop132!();
        nop132!();
    }};
}

pub(crate) use nop264;

macro_rules! nop265 {
    () => {{
        use core::arch::asm;
        nop132!();
        nop132!();
        asm!("nop");
    }};
}

pub(crate) use nop265;

macro_rules! nop266 {
    () => {{
        use core::arch::asm;
        nop133!();
        nop133!();
    }};
}

pub(crate) use nop266;

macro_rules! nop267 {
    () => {{
        use core::arch::asm;
        nop133!();
        nop133!();
        asm!("nop");
    }};
}

pub(crate) use nop267;

macro_rules! nop268 {
    () => {{
        use core::arch::asm;
        nop134!();
        nop134!();
    }};
}

pub(crate) use nop268;

macro_rules! nop269 {
    () => {{
        use core::arch::asm;
        nop134!();
        nop134!();
        asm!("nop");
    }};
}

pub(crate) use nop269;

macro_rules! nop270 {
    () => {{
        use core::arch::asm;
        nop135!();
        nop135!();
    }};
}

pub(crate) use nop270;

macro_rules! nop271 {
    () => {{
        use core::arch::asm;
        nop135!();
        nop135!();
        asm!("nop");
    }};
}

pub(crate) use nop271;

macro_rules! nop272 {
    () => {{
        use core::arch::asm;
        nop136!();
        nop136!();
    }};
}

pub(crate) use nop272;

macro_rules! nop273 {
    () => {{
        use core::arch::asm;
        nop136!();
        nop136!();
        asm!("nop");
    }};
}

pub(crate) use nop273;

macro_rules! nop274 {
    () => {{
        use core::arch::asm;
        nop137!();
        nop137!();
    }};
}

pub(crate) use nop274;

macro_rules! nop275 {
    () => {{
        use core::arch::asm;
        nop137!();
        nop137!();
        asm!("nop");
    }};
}

pub(crate) use nop275;

macro_rules! nop276 {
    () => {{
        use core::arch::asm;
        nop138!();
        nop138!();
    }};
}

pub(crate) use nop276;

macro_rules! nop277 {
    () => {{
        use core::arch::asm;
        nop138!();
        nop138!();
        asm!("nop");
    }};
}

pub(crate) use nop277;

macro_rules! nop278 {
    () => {{
        use core::arch::asm;
        nop139!();
        nop139!();
    }};
}

pub(crate) use nop278;

macro_rules! nop279 {
    () => {{
        use core::arch::asm;
        nop139!();
        nop139!();
        asm!("nop");
    }};
}

pub(crate) use nop279;

macro_rules! nop280 {
    () => {{
        use core::arch::asm;
        nop140!();
        nop140!();
    }};
}

pub(crate) use nop280;

macro_rules! nop281 {
    () => {{
        use core::arch::asm;
        nop140!();
        nop140!();
        asm!("nop");
    }};
}

pub(crate) use nop281;

macro_rules! nop282 {
    () => {{
        use core::arch::asm;
        nop141!();
        nop141!();
    }};
}

pub(crate) use nop282;

macro_rules! nop283 {
    () => {{
        use core::arch::asm;
        nop141!();
        nop141!();
        asm!("nop");
    }};
}

pub(crate) use nop283;

macro_rules! nop284 {
    () => {{
        use core::arch::asm;
        nop142!();
        nop142!();
    }};
}

pub(crate) use nop284;

macro_rules! nop285 {
    () => {{
        use core::arch::asm;
        nop142!();
        nop142!();
        asm!("nop");
    }};
}

pub(crate) use nop285;

macro_rules! nop286 {
    () => {{
        use core::arch::asm;
        nop143!();
        nop143!();
    }};
}

pub(crate) use nop286;

macro_rules! nop287 {
    () => {{
        use core::arch::asm;
        nop143!();
        nop143!();
        asm!("nop");
    }};
}

pub(crate) use nop287;

macro_rules! nop288 {
    () => {{
        use core::arch::asm;
        nop144!();
        nop144!();
    }};
}

pub(crate) use nop288;

macro_rules! nop289 {
    () => {{
        use core::arch::asm;
        nop144!();
        nop144!();
        asm!("nop");
    }};
}

pub(crate) use nop289;

macro_rules! nop290 {
    () => {{
        use core::arch::asm;
        nop145!();
        nop145!();
    }};
}

pub(crate) use nop290;

macro_rules! nop291 {
    () => {{
        use core::arch::asm;
        nop145!();
        nop145!();
        asm!("nop");
    }};
}

pub(crate) use nop291;

macro_rules! nop292 {
    () => {{
        use core::arch::asm;
        nop146!();
        nop146!();
    }};
}

pub(crate) use nop292;

macro_rules! nop293 {
    () => {{
        use core::arch::asm;
        nop146!();
        nop146!();
        asm!("nop");
    }};
}

pub(crate) use nop293;

macro_rules! nop294 {
    () => {{
        use core::arch::asm;
        nop147!();
        nop147!();
    }};
}

pub(crate) use nop294;

macro_rules! nop295 {
    () => {{
        use core::arch::asm;
        nop147!();
        nop147!();
        asm!("nop");
    }};
}

pub(crate) use nop295;

macro_rules! nop296 {
    () => {{
        use core::arch::asm;
        nop148!();
        nop148!();
    }};
}

pub(crate) use nop296;

macro_rules! nop297 {
    () => {{
        use core::arch::asm;
        nop148!();
        nop148!();
        asm!("nop");
    }};
}

pub(crate) use nop297;

macro_rules! nop298 {
    () => {{
        use core::arch::asm;
        nop149!();
        nop149!();
    }};
}

pub(crate) use nop298;

macro_rules! nop299 {
    () => {{
        use core::arch::asm;
        nop149!();
        nop149!();
        asm!("nop");
    }};
}

pub(crate) use nop299;

macro_rules! nop300 {
    () => {{
        use core::arch::asm;
        nop150!();
        nop150!();
    }};
}

pub(crate) use nop300;

macro_rules! nop301 {
    () => {{
        use core::arch::asm;
        nop150!();
        nop150!();
        asm!("nop");
    }};
}

pub(crate) use nop301;

macro_rules! nop302 {
    () => {{
        use core::arch::asm;
        nop151!();
        nop151!();
    }};
}

pub(crate) use nop302;

macro_rules! nop303 {
    () => {{
        use core::arch::asm;
        nop151!();
        nop151!();
        asm!("nop");
    }};
}

pub(crate) use nop303;

macro_rules! nop304 {
    () => {{
        use core::arch::asm;
        nop152!();
        nop152!();
    }};
}

pub(crate) use nop304;

macro_rules! nop305 {
    () => {{
        use core::arch::asm;
        nop152!();
        nop152!();
        asm!("nop");
    }};
}

pub(crate) use nop305;

macro_rules! nop306 {
    () => {{
        use core::arch::asm;
        nop153!();
        nop153!();
    }};
}

pub(crate) use nop306;

macro_rules! nop307 {
    () => {{
        use core::arch::asm;
        nop153!();
        nop153!();
        asm!("nop");
    }};
}

pub(crate) use nop307;

macro_rules! nop308 {
    () => {{
        use core::arch::asm;
        nop154!();
        nop154!();
    }};
}

pub(crate) use nop308;

macro_rules! nop309 {
    () => {{
        use core::arch::asm;
        nop154!();
        nop154!();
        asm!("nop");
    }};
}

pub(crate) use nop309;

macro_rules! nop310 {
    () => {{
        use core::arch::asm;
        nop155!();
        nop155!();
    }};
}

pub(crate) use nop310;

macro_rules! nop311 {
    () => {{
        use core::arch::asm;
        nop155!();
        nop155!();
        asm!("nop");
    }};
}

pub(crate) use nop311;

macro_rules! nop312 {
    () => {{
        use core::arch::asm;
        nop156!();
        nop156!();
    }};
}

pub(crate) use nop312;

macro_rules! nop313 {
    () => {{
        use core::arch::asm;
        nop156!();
        nop156!();
        asm!("nop");
    }};
}

pub(crate) use nop313;

macro_rules! nop314 {
    () => {{
        use core::arch::asm;
        nop157!();
        nop157!();
    }};
}

pub(crate) use nop314;

macro_rules! nop315 {
    () => {{
        use core::arch::asm;
        nop157!();
        nop157!();
        asm!("nop");
    }};
}

pub(crate) use nop315;

macro_rules! nop316 {
    () => {{
        use core::arch::asm;
        nop158!();
        nop158!();
    }};
}

pub(crate) use nop316;

macro_rules! nop317 {
    () => {{
        use core::arch::asm;
        nop158!();
        nop158!();
        asm!("nop");
    }};
}

pub(crate) use nop317;

macro_rules! nop318 {
    () => {{
        use core::arch::asm;
        nop159!();
        nop159!();
    }};
}

pub(crate) use nop318;

macro_rules! nop319 {
    () => {{
        use core::arch::asm;
        nop159!();
        nop159!();
        asm!("nop");
    }};
}

pub(crate) use nop319;

macro_rules! nop320 {
    () => {{
        use core::arch::asm;
        nop160!();
        nop160!();
    }};
}

pub(crate) use nop320;

macro_rules! nop321 {
    () => {{
        use core::arch::asm;
        nop160!();
        nop160!();
        asm!("nop");
    }};
}

pub(crate) use nop321;

macro_rules! nop322 {
    () => {{
        use core::arch::asm;
        nop161!();
        nop161!();
    }};
}

pub(crate) use nop322;

macro_rules! nop323 {
    () => {{
        use core::arch::asm;
        nop161!();
        nop161!();
        asm!("nop");
    }};
}

pub(crate) use nop323;

macro_rules! nop324 {
    () => {{
        use core::arch::asm;
        nop162!();
        nop162!();
    }};
}

pub(crate) use nop324;

macro_rules! nop325 {
    () => {{
        use core::arch::asm;
        nop162!();
        nop162!();
        asm!("nop");
    }};
}

pub(crate) use nop325;

macro_rules! nop326 {
    () => {{
        use core::arch::asm;
        nop163!();
        nop163!();
    }};
}

pub(crate) use nop326;

macro_rules! nop327 {
    () => {{
        use core::arch::asm;
        nop163!();
        nop163!();
        asm!("nop");
    }};
}

pub(crate) use nop327;

macro_rules! nop328 {
    () => {{
        use core::arch::asm;
        nop164!();
        nop164!();
    }};
}

pub(crate) use nop328;

macro_rules! nop329 {
    () => {{
        use core::arch::asm;
        nop164!();
        nop164!();
        asm!("nop");
    }};
}

pub(crate) use nop329;

macro_rules! nop330 {
    () => {{
        use core::arch::asm;
        nop165!();
        nop165!();
    }};
}

pub(crate) use nop330;

macro_rules! nop331 {
    () => {{
        use core::arch::asm;
        nop165!();
        nop165!();
        asm!("nop");
    }};
}

pub(crate) use nop331;

macro_rules! nop332 {
    () => {{
        use core::arch::asm;
        nop166!();
        nop166!();
    }};
}

pub(crate) use nop332;

macro_rules! nop333 {
    () => {{
        use core::arch::asm;
        nop166!();
        nop166!();
        asm!("nop");
    }};
}

pub(crate) use nop333;

macro_rules! nop334 {
    () => {{
        use core::arch::asm;
        nop167!();
        nop167!();
    }};
}

pub(crate) use nop334;

macro_rules! nop335 {
    () => {{
        use core::arch::asm;
        nop167!();
        nop167!();
        asm!("nop");
    }};
}

pub(crate) use nop335;

macro_rules! nop336 {
    () => {{
        use core::arch::asm;
        nop168!();
        nop168!();
    }};
}

pub(crate) use nop336;

macro_rules! nop337 {
    () => {{
        use core::arch::asm;
        nop168!();
        nop168!();
        asm!("nop");
    }};
}

pub(crate) use nop337;

macro_rules! nop338 {
    () => {{
        use core::arch::asm;
        nop169!();
        nop169!();
    }};
}

pub(crate) use nop338;

macro_rules! nop339 {
    () => {{
        use core::arch::asm;
        nop169!();
        nop169!();
        asm!("nop");
    }};
}

pub(crate) use nop339;

macro_rules! nop340 {
    () => {{
        use core::arch::asm;
        nop170!();
        nop170!();
    }};
}

pub(crate) use nop340;

macro_rules! nop341 {
    () => {{
        use core::arch::asm;
        nop170!();
        nop170!();
        asm!("nop");
    }};
}

pub(crate) use nop341;

macro_rules! nop342 {
    () => {{
        use core::arch::asm;
        nop171!();
        nop171!();
    }};
}

pub(crate) use nop342;

macro_rules! nop343 {
    () => {{
        use core::arch::asm;
        nop171!();
        nop171!();
        asm!("nop");
    }};
}

pub(crate) use nop343;

macro_rules! nop344 {
    () => {{
        use core::arch::asm;
        nop172!();
        nop172!();
    }};
}

pub(crate) use nop344;

macro_rules! nop345 {
    () => {{
        use core::arch::asm;
        nop172!();
        nop172!();
        asm!("nop");
    }};
}

pub(crate) use nop345;

macro_rules! nop346 {
    () => {{
        use core::arch::asm;
        nop173!();
        nop173!();
    }};
}

pub(crate) use nop346;

macro_rules! nop347 {
    () => {{
        use core::arch::asm;
        nop173!();
        nop173!();
        asm!("nop");
    }};
}

pub(crate) use nop347;

macro_rules! nop348 {
    () => {{
        use core::arch::asm;
        nop174!();
        nop174!();
    }};
}

pub(crate) use nop348;

macro_rules! nop349 {
    () => {{
        use core::arch::asm;
        nop174!();
        nop174!();
        asm!("nop");
    }};
}

pub(crate) use nop349;

macro_rules! nop350 {
    () => {{
        use core::arch::asm;
        nop175!();
        nop175!();
    }};
}

pub(crate) use nop350;

macro_rules! nop351 {
    () => {{
        use core::arch::asm;
        nop175!();
        nop175!();
        asm!("nop");
    }};
}

pub(crate) use nop351;

macro_rules! nop352 {
    () => {{
        use core::arch::asm;
        nop176!();
        nop176!();
    }};
}

pub(crate) use nop352;

macro_rules! nop353 {
    () => {{
        use core::arch::asm;
        nop176!();
        nop176!();
        asm!("nop");
    }};
}

pub(crate) use nop353;

macro_rules! nop354 {
    () => {{
        use core::arch::asm;
        nop177!();
        nop177!();
    }};
}

pub(crate) use nop354;

macro_rules! nop355 {
    () => {{
        use core::arch::asm;
        nop177!();
        nop177!();
        asm!("nop");
    }};
}

pub(crate) use nop355;

macro_rules! nop356 {
    () => {{
        use core::arch::asm;
        nop178!();
        nop178!();
    }};
}

pub(crate) use nop356;

macro_rules! nop357 {
    () => {{
        use core::arch::asm;
        nop178!();
        nop178!();
        asm!("nop");
    }};
}

pub(crate) use nop357;

macro_rules! nop358 {
    () => {{
        use core::arch::asm;
        nop179!();
        nop179!();
    }};
}

pub(crate) use nop358;

macro_rules! nop359 {
    () => {{
        use core::arch::asm;
        nop179!();
        nop179!();
        asm!("nop");
    }};
}

pub(crate) use nop359;

macro_rules! nop360 {
    () => {{
        use core::arch::asm;
        nop180!();
        nop180!();
    }};
}

pub(crate) use nop360;

macro_rules! nop361 {
    () => {{
        use core::arch::asm;
        nop180!();
        nop180!();
        asm!("nop");
    }};
}

pub(crate) use nop361;

macro_rules! nop362 {
    () => {{
        use core::arch::asm;
        nop181!();
        nop181!();
    }};
}

pub(crate) use nop362;

macro_rules! nop363 {
    () => {{
        use core::arch::asm;
        nop181!();
        nop181!();
        asm!("nop");
    }};
}

pub(crate) use nop363;

macro_rules! nop364 {
    () => {{
        use core::arch::asm;
        nop182!();
        nop182!();
    }};
}

pub(crate) use nop364;

macro_rules! nop365 {
    () => {{
        use core::arch::asm;
        nop182!();
        nop182!();
        asm!("nop");
    }};
}

pub(crate) use nop365;

macro_rules! nop366 {
    () => {{
        use core::arch::asm;
        nop183!();
        nop183!();
    }};
}

pub(crate) use nop366;

macro_rules! nop367 {
    () => {{
        use core::arch::asm;
        nop183!();
        nop183!();
        asm!("nop");
    }};
}

pub(crate) use nop367;

macro_rules! nop368 {
    () => {{
        use core::arch::asm;
        nop184!();
        nop184!();
    }};
}

pub(crate) use nop368;

macro_rules! nop369 {
    () => {{
        use core::arch::asm;
        nop184!();
        nop184!();
        asm!("nop");
    }};
}

pub(crate) use nop369;

macro_rules! nop370 {
    () => {{
        use core::arch::asm;
        nop185!();
        nop185!();
    }};
}

pub(crate) use nop370;

macro_rules! nop371 {
    () => {{
        use core::arch::asm;
        nop185!();
        nop185!();
        asm!("nop");
    }};
}

pub(crate) use nop371;

macro_rules! nop372 {
    () => {{
        use core::arch::asm;
        nop186!();
        nop186!();
    }};
}

pub(crate) use nop372;

macro_rules! nop373 {
    () => {{
        use core::arch::asm;
        nop186!();
        nop186!();
        asm!("nop");
    }};
}

pub(crate) use nop373;

macro_rules! nop374 {
    () => {{
        use core::arch::asm;
        nop187!();
        nop187!();
    }};
}

pub(crate) use nop374;

macro_rules! nop375 {
    () => {{
        use core::arch::asm;
        nop187!();
        nop187!();
        asm!("nop");
    }};
}

pub(crate) use nop375;

macro_rules! nop376 {
    () => {{
        use core::arch::asm;
        nop188!();
        nop188!();
    }};
}

pub(crate) use nop376;

macro_rules! nop377 {
    () => {{
        use core::arch::asm;
        nop188!();
        nop188!();
        asm!("nop");
    }};
}

pub(crate) use nop377;

macro_rules! nop378 {
    () => {{
        use core::arch::asm;
        nop189!();
        nop189!();
    }};
}

pub(crate) use nop378;

macro_rules! nop379 {
    () => {{
        use core::arch::asm;
        nop189!();
        nop189!();
        asm!("nop");
    }};
}

pub(crate) use nop379;

macro_rules! nop380 {
    () => {{
        use core::arch::asm;
        nop190!();
        nop190!();
    }};
}

pub(crate) use nop380;

macro_rules! nop381 {
    () => {{
        use core::arch::asm;
        nop190!();
        nop190!();
        asm!("nop");
    }};
}

pub(crate) use nop381;

macro_rules! nop382 {
    () => {{
        use core::arch::asm;
        nop191!();
        nop191!();
    }};
}

pub(crate) use nop382;

macro_rules! nop383 {
    () => {{
        use core::arch::asm;
        nop191!();
        nop191!();
        asm!("nop");
    }};
}

pub(crate) use nop383;

macro_rules! nop384 {
    () => {{
        use core::arch::asm;
        nop192!();
        nop192!();
    }};
}

pub(crate) use nop384;

macro_rules! nop385 {
    () => {{
        use core::arch::asm;
        nop192!();
        nop192!();
        asm!("nop");
    }};
}

pub(crate) use nop385;

macro_rules! nop386 {
    () => {{
        use core::arch::asm;
        nop193!();
        nop193!();
    }};
}

pub(crate) use nop386;

macro_rules! nop387 {
    () => {{
        use core::arch::asm;
        nop193!();
        nop193!();
        asm!("nop");
    }};
}

pub(crate) use nop387;

macro_rules! nop388 {
    () => {{
        use core::arch::asm;
        nop194!();
        nop194!();
    }};
}

pub(crate) use nop388;

macro_rules! nop389 {
    () => {{
        use core::arch::asm;
        nop194!();
        nop194!();
        asm!("nop");
    }};
}

pub(crate) use nop389;

macro_rules! nop390 {
    () => {{
        use core::arch::asm;
        nop195!();
        nop195!();
    }};
}

pub(crate) use nop390;

macro_rules! nop391 {
    () => {{
        use core::arch::asm;
        nop195!();
        nop195!();
        asm!("nop");
    }};
}

pub(crate) use nop391;

macro_rules! nop392 {
    () => {{
        use core::arch::asm;
        nop196!();
        nop196!();
    }};
}

pub(crate) use nop392;

macro_rules! nop393 {
    () => {{
        use core::arch::asm;
        nop196!();
        nop196!();
        asm!("nop");
    }};
}

pub(crate) use nop393;

macro_rules! nop394 {
    () => {{
        use core::arch::asm;
        nop197!();
        nop197!();
    }};
}

pub(crate) use nop394;

macro_rules! nop395 {
    () => {{
        use core::arch::asm;
        nop197!();
        nop197!();
        asm!("nop");
    }};
}

pub(crate) use nop395;

macro_rules! nop396 {
    () => {{
        use core::arch::asm;
        nop198!();
        nop198!();
    }};
}

pub(crate) use nop396;

macro_rules! nop397 {
    () => {{
        use core::arch::asm;
        nop198!();
        nop198!();
        asm!("nop");
    }};
}

pub(crate) use nop397;

macro_rules! nop398 {
    () => {{
        use core::arch::asm;
        nop199!();
        nop199!();
    }};
}

pub(crate) use nop398;

macro_rules! nop399 {
    () => {{
        use core::arch::asm;
        nop199!();
        nop199!();
        asm!("nop");
    }};
}

pub(crate) use nop399;

macro_rules! nop400 {
    () => {{
        use core::arch::asm;
        nop200!();
        nop200!();
    }};
}

pub(crate) use nop400;

macro_rules! nop401 {
    () => {{
        use core::arch::asm;
        nop200!();
        nop200!();
        asm!("nop");
    }};
}

pub(crate) use nop401;

macro_rules! nop402 {
    () => {{
        use core::arch::asm;
        nop201!();
        nop201!();
    }};
}

pub(crate) use nop402;

macro_rules! nop403 {
    () => {{
        use core::arch::asm;
        nop201!();
        nop201!();
        asm!("nop");
    }};
}

pub(crate) use nop403;

macro_rules! nop404 {
    () => {{
        use core::arch::asm;
        nop202!();
        nop202!();
    }};
}

pub(crate) use nop404;

macro_rules! nop405 {
    () => {{
        use core::arch::asm;
        nop202!();
        nop202!();
        asm!("nop");
    }};
}

pub(crate) use nop405;

macro_rules! nop406 {
    () => {{
        use core::arch::asm;
        nop203!();
        nop203!();
    }};
}

pub(crate) use nop406;

macro_rules! nop407 {
    () => {{
        use core::arch::asm;
        nop203!();
        nop203!();
        asm!("nop");
    }};
}

pub(crate) use nop407;

macro_rules! nop408 {
    () => {{
        use core::arch::asm;
        nop204!();
        nop204!();
    }};
}

pub(crate) use nop408;

macro_rules! nop409 {
    () => {{
        use core::arch::asm;
        nop204!();
        nop204!();
        asm!("nop");
    }};
}

pub(crate) use nop409;

macro_rules! nop410 {
    () => {{
        use core::arch::asm;
        nop205!();
        nop205!();
    }};
}

pub(crate) use nop410;

macro_rules! nop411 {
    () => {{
        use core::arch::asm;
        nop205!();
        nop205!();
        asm!("nop");
    }};
}

pub(crate) use nop411;

macro_rules! nop412 {
    () => {{
        use core::arch::asm;
        nop206!();
        nop206!();
    }};
}

pub(crate) use nop412;

macro_rules! nop413 {
    () => {{
        use core::arch::asm;
        nop206!();
        nop206!();
        asm!("nop");
    }};
}

pub(crate) use nop413;

macro_rules! nop414 {
    () => {{
        use core::arch::asm;
        nop207!();
        nop207!();
    }};
}

pub(crate) use nop414;

macro_rules! nop415 {
    () => {{
        use core::arch::asm;
        nop207!();
        nop207!();
        asm!("nop");
    }};
}

pub(crate) use nop415;

macro_rules! nop416 {
    () => {{
        use core::arch::asm;
        nop208!();
        nop208!();
    }};
}

pub(crate) use nop416;

macro_rules! nop417 {
    () => {{
        use core::arch::asm;
        nop208!();
        nop208!();
        asm!("nop");
    }};
}

pub(crate) use nop417;

macro_rules! nop418 {
    () => {{
        use core::arch::asm;
        nop209!();
        nop209!();
    }};
}

pub(crate) use nop418;

macro_rules! nop419 {
    () => {{
        use core::arch::asm;
        nop209!();
        nop209!();
        asm!("nop");
    }};
}

pub(crate) use nop419;

macro_rules! nop420 {
    () => {{
        use core::arch::asm;
        nop210!();
        nop210!();
    }};
}

pub(crate) use nop420;

macro_rules! nop421 {
    () => {{
        use core::arch::asm;
        nop210!();
        nop210!();
        asm!("nop");
    }};
}

pub(crate) use nop421;

macro_rules! nop422 {
    () => {{
        use core::arch::asm;
        nop211!();
        nop211!();
    }};
}

pub(crate) use nop422;

macro_rules! nop423 {
    () => {{
        use core::arch::asm;
        nop211!();
        nop211!();
        asm!("nop");
    }};
}

pub(crate) use nop423;

macro_rules! nop424 {
    () => {{
        use core::arch::asm;
        nop212!();
        nop212!();
    }};
}

pub(crate) use nop424;

macro_rules! nop425 {
    () => {{
        use core::arch::asm;
        nop212!();
        nop212!();
        asm!("nop");
    }};
}

pub(crate) use nop425;

macro_rules! nop426 {
    () => {{
        use core::arch::asm;
        nop213!();
        nop213!();
    }};
}

pub(crate) use nop426;

macro_rules! nop427 {
    () => {{
        use core::arch::asm;
        nop213!();
        nop213!();
        asm!("nop");
    }};
}

pub(crate) use nop427;

macro_rules! nop428 {
    () => {{
        use core::arch::asm;
        nop214!();
        nop214!();
    }};
}

pub(crate) use nop428;

macro_rules! nop429 {
    () => {{
        use core::arch::asm;
        nop214!();
        nop214!();
        asm!("nop");
    }};
}

pub(crate) use nop429;

macro_rules! nop430 {
    () => {{
        use core::arch::asm;
        nop215!();
        nop215!();
    }};
}

pub(crate) use nop430;

macro_rules! nop431 {
    () => {{
        use core::arch::asm;
        nop215!();
        nop215!();
        asm!("nop");
    }};
}

pub(crate) use nop431;

macro_rules! nop432 {
    () => {{
        use core::arch::asm;
        nop216!();
        nop216!();
    }};
}

pub(crate) use nop432;

macro_rules! nop433 {
    () => {{
        use core::arch::asm;
        nop216!();
        nop216!();
        asm!("nop");
    }};
}

pub(crate) use nop433;

macro_rules! nop434 {
    () => {{
        use core::arch::asm;
        nop217!();
        nop217!();
    }};
}

pub(crate) use nop434;

macro_rules! nop435 {
    () => {{
        use core::arch::asm;
        nop217!();
        nop217!();
        asm!("nop");
    }};
}

pub(crate) use nop435;

macro_rules! nop436 {
    () => {{
        use core::arch::asm;
        nop218!();
        nop218!();
    }};
}

pub(crate) use nop436;

macro_rules! nop437 {
    () => {{
        use core::arch::asm;
        nop218!();
        nop218!();
        asm!("nop");
    }};
}

pub(crate) use nop437;

macro_rules! nop438 {
    () => {{
        use core::arch::asm;
        nop219!();
        nop219!();
    }};
}

pub(crate) use nop438;

macro_rules! nop439 {
    () => {{
        use core::arch::asm;
        nop219!();
        nop219!();
        asm!("nop");
    }};
}

pub(crate) use nop439;

macro_rules! nop440 {
    () => {{
        use core::arch::asm;
        nop220!();
        nop220!();
    }};
}

pub(crate) use nop440;

macro_rules! nop441 {
    () => {{
        use core::arch::asm;
        nop220!();
        nop220!();
        asm!("nop");
    }};
}

pub(crate) use nop441;

macro_rules! nop442 {
    () => {{
        use core::arch::asm;
        nop221!();
        nop221!();
    }};
}

pub(crate) use nop442;

macro_rules! nop443 {
    () => {{
        use core::arch::asm;
        nop221!();
        nop221!();
        asm!("nop");
    }};
}

pub(crate) use nop443;

macro_rules! nop444 {
    () => {{
        use core::arch::asm;
        nop222!();
        nop222!();
    }};
}

pub(crate) use nop444;

macro_rules! nop445 {
    () => {{
        use core::arch::asm;
        nop222!();
        nop222!();
        asm!("nop");
    }};
}

pub(crate) use nop445;

macro_rules! nop446 {
    () => {{
        use core::arch::asm;
        nop223!();
        nop223!();
    }};
}

pub(crate) use nop446;

macro_rules! nop447 {
    () => {{
        use core::arch::asm;
        nop223!();
        nop223!();
        asm!("nop");
    }};
}

pub(crate) use nop447;

macro_rules! nop448 {
    () => {{
        use core::arch::asm;
        nop224!();
        nop224!();
    }};
}

pub(crate) use nop448;

macro_rules! nop449 {
    () => {{
        use core::arch::asm;
        nop224!();
        nop224!();
        asm!("nop");
    }};
}

pub(crate) use nop449;

macro_rules! nop450 {
    () => {{
        use core::arch::asm;
        nop225!();
        nop225!();
    }};
}

pub(crate) use nop450;

macro_rules! nop451 {
    () => {{
        use core::arch::asm;
        nop225!();
        nop225!();
        asm!("nop");
    }};
}

pub(crate) use nop451;

macro_rules! nop452 {
    () => {{
        use core::arch::asm;
        nop226!();
        nop226!();
    }};
}

pub(crate) use nop452;

macro_rules! nop453 {
    () => {{
        use core::arch::asm;
        nop226!();
        nop226!();
        asm!("nop");
    }};
}

pub(crate) use nop453;

macro_rules! nop454 {
    () => {{
        use core::arch::asm;
        nop227!();
        nop227!();
    }};
}

pub(crate) use nop454;

macro_rules! nop455 {
    () => {{
        use core::arch::asm;
        nop227!();
        nop227!();
        asm!("nop");
    }};
}

pub(crate) use nop455;

macro_rules! nop456 {
    () => {{
        use core::arch::asm;
        nop228!();
        nop228!();
    }};
}

pub(crate) use nop456;

macro_rules! nop457 {
    () => {{
        use core::arch::asm;
        nop228!();
        nop228!();
        asm!("nop");
    }};
}

pub(crate) use nop457;

macro_rules! nop458 {
    () => {{
        use core::arch::asm;
        nop229!();
        nop229!();
    }};
}

pub(crate) use nop458;

macro_rules! nop459 {
    () => {{
        use core::arch::asm;
        nop229!();
        nop229!();
        asm!("nop");
    }};
}

pub(crate) use nop459;

macro_rules! nop460 {
    () => {{
        use core::arch::asm;
        nop230!();
        nop230!();
    }};
}

pub(crate) use nop460;

macro_rules! nop461 {
    () => {{
        use core::arch::asm;
        nop230!();
        nop230!();
        asm!("nop");
    }};
}

pub(crate) use nop461;

macro_rules! nop462 {
    () => {{
        use core::arch::asm;
        nop231!();
        nop231!();
    }};
}

pub(crate) use nop462;

macro_rules! nop463 {
    () => {{
        use core::arch::asm;
        nop231!();
        nop231!();
        asm!("nop");
    }};
}

pub(crate) use nop463;

macro_rules! nop464 {
    () => {{
        use core::arch::asm;
        nop232!();
        nop232!();
    }};
}

pub(crate) use nop464;

macro_rules! nop465 {
    () => {{
        use core::arch::asm;
        nop232!();
        nop232!();
        asm!("nop");
    }};
}

pub(crate) use nop465;

macro_rules! nop466 {
    () => {{
        use core::arch::asm;
        nop233!();
        nop233!();
    }};
}

pub(crate) use nop466;

macro_rules! nop467 {
    () => {{
        use core::arch::asm;
        nop233!();
        nop233!();
        asm!("nop");
    }};
}

pub(crate) use nop467;

macro_rules! nop468 {
    () => {{
        use core::arch::asm;
        nop234!();
        nop234!();
    }};
}

pub(crate) use nop468;

macro_rules! nop469 {
    () => {{
        use core::arch::asm;
        nop234!();
        nop234!();
        asm!("nop");
    }};
}

pub(crate) use nop469;

macro_rules! nop470 {
    () => {{
        use core::arch::asm;
        nop235!();
        nop235!();
    }};
}

pub(crate) use nop470;

macro_rules! nop471 {
    () => {{
        use core::arch::asm;
        nop235!();
        nop235!();
        asm!("nop");
    }};
}

pub(crate) use nop471;

macro_rules! nop472 {
    () => {{
        use core::arch::asm;
        nop236!();
        nop236!();
    }};
}

pub(crate) use nop472;

macro_rules! nop473 {
    () => {{
        use core::arch::asm;
        nop236!();
        nop236!();
        asm!("nop");
    }};
}

pub(crate) use nop473;

macro_rules! nop474 {
    () => {{
        use core::arch::asm;
        nop237!();
        nop237!();
    }};
}

pub(crate) use nop474;

macro_rules! nop475 {
    () => {{
        use core::arch::asm;
        nop237!();
        nop237!();
        asm!("nop");
    }};
}

pub(crate) use nop475;

macro_rules! nop476 {
    () => {{
        use core::arch::asm;
        nop238!();
        nop238!();
    }};
}

pub(crate) use nop476;

macro_rules! nop477 {
    () => {{
        use core::arch::asm;
        nop238!();
        nop238!();
        asm!("nop");
    }};
}

pub(crate) use nop477;

macro_rules! nop478 {
    () => {{
        use core::arch::asm;
        nop239!();
        nop239!();
    }};
}

pub(crate) use nop478;

macro_rules! nop479 {
    () => {{
        use core::arch::asm;
        nop239!();
        nop239!();
        asm!("nop");
    }};
}

pub(crate) use nop479;

macro_rules! nop480 {
    () => {{
        use core::arch::asm;
        nop240!();
        nop240!();
    }};
}

pub(crate) use nop480;

macro_rules! nop481 {
    () => {{
        use core::arch::asm;
        nop240!();
        nop240!();
        asm!("nop");
    }};
}

pub(crate) use nop481;

macro_rules! nop482 {
    () => {{
        use core::arch::asm;
        nop241!();
        nop241!();
    }};
}

pub(crate) use nop482;

macro_rules! nop483 {
    () => {{
        use core::arch::asm;
        nop241!();
        nop241!();
        asm!("nop");
    }};
}

pub(crate) use nop483;

macro_rules! nop484 {
    () => {{
        use core::arch::asm;
        nop242!();
        nop242!();
    }};
}

pub(crate) use nop484;

macro_rules! nop485 {
    () => {{
        use core::arch::asm;
        nop242!();
        nop242!();
        asm!("nop");
    }};
}

pub(crate) use nop485;

macro_rules! nop486 {
    () => {{
        use core::arch::asm;
        nop243!();
        nop243!();
    }};
}

pub(crate) use nop486;

macro_rules! nop487 {
    () => {{
        use core::arch::asm;
        nop243!();
        nop243!();
        asm!("nop");
    }};
}

pub(crate) use nop487;

macro_rules! nop488 {
    () => {{
        use core::arch::asm;
        nop244!();
        nop244!();
    }};
}

pub(crate) use nop488;

macro_rules! nop489 {
    () => {{
        use core::arch::asm;
        nop244!();
        nop244!();
        asm!("nop");
    }};
}

pub(crate) use nop489;

macro_rules! nop490 {
    () => {{
        use core::arch::asm;
        nop245!();
        nop245!();
    }};
}

pub(crate) use nop490;

macro_rules! nop491 {
    () => {{
        use core::arch::asm;
        nop245!();
        nop245!();
        asm!("nop");
    }};
}

pub(crate) use nop491;

macro_rules! nop492 {
    () => {{
        use core::arch::asm;
        nop246!();
        nop246!();
    }};
}

pub(crate) use nop492;

macro_rules! nop493 {
    () => {{
        use core::arch::asm;
        nop246!();
        nop246!();
        asm!("nop");
    }};
}

pub(crate) use nop493;

macro_rules! nop494 {
    () => {{
        use core::arch::asm;
        nop247!();
        nop247!();
    }};
}

pub(crate) use nop494;

macro_rules! nop495 {
    () => {{
        use core::arch::asm;
        nop247!();
        nop247!();
        asm!("nop");
    }};
}

pub(crate) use nop495;

macro_rules! nop496 {
    () => {{
        use core::arch::asm;
        nop248!();
        nop248!();
    }};
}

pub(crate) use nop496;

macro_rules! nop497 {
    () => {{
        use core::arch::asm;
        nop248!();
        nop248!();
        asm!("nop");
    }};
}

pub(crate) use nop497;

macro_rules! nop498 {
    () => {{
        use core::arch::asm;
        nop249!();
        nop249!();
    }};
}

pub(crate) use nop498;

macro_rules! nop499 {
    () => {{
        use core::arch::asm;
        nop249!();
        nop249!();
        asm!("nop");
    }};
}

pub(crate) use nop499;

macro_rules! nop500 {
    () => {{
        use core::arch::asm;
        nop250!();
        nop250!();
    }};
}

pub(crate) use nop500;

macro_rules! nop501 {
    () => {{
        use core::arch::asm;
        nop250!();
        nop250!();
        asm!("nop");
    }};
}

pub(crate) use nop501;

macro_rules! nop502 {
    () => {{
        use core::arch::asm;
        nop251!();
        nop251!();
    }};
}

pub(crate) use nop502;

macro_rules! nop503 {
    () => {{
        use core::arch::asm;
        nop251!();
        nop251!();
        asm!("nop");
    }};
}

pub(crate) use nop503;

macro_rules! nop504 {
    () => {{
        use core::arch::asm;
        nop252!();
        nop252!();
    }};
}

pub(crate) use nop504;

macro_rules! nop505 {
    () => {{
        use core::arch::asm;
        nop252!();
        nop252!();
        asm!("nop");
    }};
}

pub(crate) use nop505;

macro_rules! nop506 {
    () => {{
        use core::arch::asm;
        nop253!();
        nop253!();
    }};
}

pub(crate) use nop506;

macro_rules! nop507 {
    () => {{
        use core::arch::asm;
        nop253!();
        nop253!();
        asm!("nop");
    }};
}

pub(crate) use nop507;

macro_rules! nop508 {
    () => {{
        use core::arch::asm;
        nop254!();
        nop254!();
    }};
}

pub(crate) use nop508;

macro_rules! nop509 {
    () => {{
        use core::arch::asm;
        nop254!();
        nop254!();
        asm!("nop");
    }};
}

pub(crate) use nop509;

macro_rules! nop510 {
    () => {{
        use core::arch::asm;
        nop255!();
        nop255!();
    }};
}

pub(crate) use nop510;

macro_rules! nop511 {
    () => {{
        use core::arch::asm;
        nop255!();
        nop255!();
        asm!("nop");
    }};
}

pub(crate) use nop511;

macro_rules! nop512 {
    () => {{
        use core::arch::asm;
        nop256!();
        nop256!();
    }};
}

pub(crate) use nop512;

macro_rules! nop513 {
    () => {{
        use core::arch::asm;
        nop256!();
        nop256!();
        asm!("nop");
    }};
}

pub(crate) use nop513;

macro_rules! nop514 {
    () => {{
        use core::arch::asm;
        nop257!();
        nop257!();
    }};
}

pub(crate) use nop514;

macro_rules! nop515 {
    () => {{
        use core::arch::asm;
        nop257!();
        nop257!();
        asm!("nop");
    }};
}

pub(crate) use nop515;

macro_rules! nop516 {
    () => {{
        use core::arch::asm;
        nop258!();
        nop258!();
    }};
}

pub(crate) use nop516;

macro_rules! nop517 {
    () => {{
        use core::arch::asm;
        nop258!();
        nop258!();
        asm!("nop");
    }};
}

pub(crate) use nop517;

macro_rules! nop518 {
    () => {{
        use core::arch::asm;
        nop259!();
        nop259!();
    }};
}

pub(crate) use nop518;

macro_rules! nop519 {
    () => {{
        use core::arch::asm;
        nop259!();
        nop259!();
        asm!("nop");
    }};
}

pub(crate) use nop519;

macro_rules! nop520 {
    () => {{
        use core::arch::asm;
        nop260!();
        nop260!();
    }};
}

pub(crate) use nop520;

macro_rules! nop521 {
    () => {{
        use core::arch::asm;
        nop260!();
        nop260!();
        asm!("nop");
    }};
}

pub(crate) use nop521;

macro_rules! nop522 {
    () => {{
        use core::arch::asm;
        nop261!();
        nop261!();
    }};
}

pub(crate) use nop522;

macro_rules! nop523 {
    () => {{
        use core::arch::asm;
        nop261!();
        nop261!();
        asm!("nop");
    }};
}

pub(crate) use nop523;

macro_rules! nop524 {
    () => {{
        use core::arch::asm;
        nop262!();
        nop262!();
    }};
}

pub(crate) use nop524;

macro_rules! nop525 {
    () => {{
        use core::arch::asm;
        nop262!();
        nop262!();
        asm!("nop");
    }};
}

pub(crate) use nop525;

macro_rules! nop526 {
    () => {{
        use core::arch::asm;
        nop263!();
        nop263!();
    }};
}

pub(crate) use nop526;

macro_rules! nop527 {
    () => {{
        use core::arch::asm;
        nop263!();
        nop263!();
        asm!("nop");
    }};
}

pub(crate) use nop527;

macro_rules! nop528 {
    () => {{
        use core::arch::asm;
        nop264!();
        nop264!();
    }};
}

pub(crate) use nop528;

macro_rules! nop529 {
    () => {{
        use core::arch::asm;
        nop264!();
        nop264!();
        asm!("nop");
    }};
}

pub(crate) use nop529;

macro_rules! nop530 {
    () => {{
        use core::arch::asm;
        nop265!();
        nop265!();
    }};
}

pub(crate) use nop530;

macro_rules! nop531 {
    () => {{
        use core::arch::asm;
        nop265!();
        nop265!();
        asm!("nop");
    }};
}

pub(crate) use nop531;

macro_rules! nop532 {
    () => {{
        use core::arch::asm;
        nop266!();
        nop266!();
    }};
}

pub(crate) use nop532;

macro_rules! nop533 {
    () => {{
        use core::arch::asm;
        nop266!();
        nop266!();
        asm!("nop");
    }};
}

pub(crate) use nop533;

macro_rules! nop534 {
    () => {{
        use core::arch::asm;
        nop267!();
        nop267!();
    }};
}

pub(crate) use nop534;

macro_rules! nop535 {
    () => {{
        use core::arch::asm;
        nop267!();
        nop267!();
        asm!("nop");
    }};
}

pub(crate) use nop535;

macro_rules! nop536 {
    () => {{
        use core::arch::asm;
        nop268!();
        nop268!();
    }};
}

pub(crate) use nop536;

macro_rules! nop537 {
    () => {{
        use core::arch::asm;
        nop268!();
        nop268!();
        asm!("nop");
    }};
}

pub(crate) use nop537;

macro_rules! nop538 {
    () => {{
        use core::arch::asm;
        nop269!();
        nop269!();
    }};
}

pub(crate) use nop538;

macro_rules! nop539 {
    () => {{
        use core::arch::asm;
        nop269!();
        nop269!();
        asm!("nop");
    }};
}

pub(crate) use nop539;

macro_rules! nop540 {
    () => {{
        use core::arch::asm;
        nop270!();
        nop270!();
    }};
}

pub(crate) use nop540;

macro_rules! nop541 {
    () => {{
        use core::arch::asm;
        nop270!();
        nop270!();
        asm!("nop");
    }};
}

pub(crate) use nop541;

macro_rules! nop542 {
    () => {{
        use core::arch::asm;
        nop271!();
        nop271!();
    }};
}

pub(crate) use nop542;

macro_rules! nop543 {
    () => {{
        use core::arch::asm;
        nop271!();
        nop271!();
        asm!("nop");
    }};
}

pub(crate) use nop543;

macro_rules! nop544 {
    () => {{
        use core::arch::asm;
        nop272!();
        nop272!();
    }};
}

pub(crate) use nop544;

macro_rules! nop545 {
    () => {{
        use core::arch::asm;
        nop272!();
        nop272!();
        asm!("nop");
    }};
}

pub(crate) use nop545;

macro_rules! nop546 {
    () => {{
        use core::arch::asm;
        nop273!();
        nop273!();
    }};
}

pub(crate) use nop546;

macro_rules! nop547 {
    () => {{
        use core::arch::asm;
        nop273!();
        nop273!();
        asm!("nop");
    }};
}

pub(crate) use nop547;

macro_rules! nop548 {
    () => {{
        use core::arch::asm;
        nop274!();
        nop274!();
    }};
}

pub(crate) use nop548;

macro_rules! nop549 {
    () => {{
        use core::arch::asm;
        nop274!();
        nop274!();
        asm!("nop");
    }};
}

pub(crate) use nop549;

macro_rules! nop550 {
    () => {{
        use core::arch::asm;
        nop275!();
        nop275!();
    }};
}

pub(crate) use nop550;

macro_rules! nop551 {
    () => {{
        use core::arch::asm;
        nop275!();
        nop275!();
        asm!("nop");
    }};
}

pub(crate) use nop551;

macro_rules! nop552 {
    () => {{
        use core::arch::asm;
        nop276!();
        nop276!();
    }};
}

pub(crate) use nop552;

macro_rules! nop553 {
    () => {{
        use core::arch::asm;
        nop276!();
        nop276!();
        asm!("nop");
    }};
}

pub(crate) use nop553;

macro_rules! nop554 {
    () => {{
        use core::arch::asm;
        nop277!();
        nop277!();
    }};
}

pub(crate) use nop554;

macro_rules! nop555 {
    () => {{
        use core::arch::asm;
        nop277!();
        nop277!();
        asm!("nop");
    }};
}

pub(crate) use nop555;

macro_rules! nop556 {
    () => {{
        use core::arch::asm;
        nop278!();
        nop278!();
    }};
}

pub(crate) use nop556;

macro_rules! nop557 {
    () => {{
        use core::arch::asm;
        nop278!();
        nop278!();
        asm!("nop");
    }};
}

pub(crate) use nop557;

macro_rules! nop558 {
    () => {{
        use core::arch::asm;
        nop279!();
        nop279!();
    }};
}

pub(crate) use nop558;

macro_rules! nop559 {
    () => {{
        use core::arch::asm;
        nop279!();
        nop279!();
        asm!("nop");
    }};
}

pub(crate) use nop559;

macro_rules! nop560 {
    () => {{
        use core::arch::asm;
        nop280!();
        nop280!();
    }};
}

pub(crate) use nop560;

macro_rules! nop561 {
    () => {{
        use core::arch::asm;
        nop280!();
        nop280!();
        asm!("nop");
    }};
}

pub(crate) use nop561;

macro_rules! nop562 {
    () => {{
        use core::arch::asm;
        nop281!();
        nop281!();
    }};
}

pub(crate) use nop562;

macro_rules! nop563 {
    () => {{
        use core::arch::asm;
        nop281!();
        nop281!();
        asm!("nop");
    }};
}

pub(crate) use nop563;

macro_rules! nop564 {
    () => {{
        use core::arch::asm;
        nop282!();
        nop282!();
    }};
}

pub(crate) use nop564;

macro_rules! nop565 {
    () => {{
        use core::arch::asm;
        nop282!();
        nop282!();
        asm!("nop");
    }};
}

pub(crate) use nop565;

macro_rules! nop566 {
    () => {{
        use core::arch::asm;
        nop283!();
        nop283!();
    }};
}

pub(crate) use nop566;

macro_rules! nop567 {
    () => {{
        use core::arch::asm;
        nop283!();
        nop283!();
        asm!("nop");
    }};
}

pub(crate) use nop567;

macro_rules! nop568 {
    () => {{
        use core::arch::asm;
        nop284!();
        nop284!();
    }};
}

pub(crate) use nop568;

macro_rules! nop569 {
    () => {{
        use core::arch::asm;
        nop284!();
        nop284!();
        asm!("nop");
    }};
}

pub(crate) use nop569;

macro_rules! nop570 {
    () => {{
        use core::arch::asm;
        nop285!();
        nop285!();
    }};
}

pub(crate) use nop570;

macro_rules! nop571 {
    () => {{
        use core::arch::asm;
        nop285!();
        nop285!();
        asm!("nop");
    }};
}

pub(crate) use nop571;

macro_rules! nop572 {
    () => {{
        use core::arch::asm;
        nop286!();
        nop286!();
    }};
}

pub(crate) use nop572;

macro_rules! nop573 {
    () => {{
        use core::arch::asm;
        nop286!();
        nop286!();
        asm!("nop");
    }};
}

pub(crate) use nop573;

macro_rules! nop574 {
    () => {{
        use core::arch::asm;
        nop287!();
        nop287!();
    }};
}

pub(crate) use nop574;

macro_rules! nop575 {
    () => {{
        use core::arch::asm;
        nop287!();
        nop287!();
        asm!("nop");
    }};
}

pub(crate) use nop575;

macro_rules! nop576 {
    () => {{
        use core::arch::asm;
        nop288!();
        nop288!();
    }};
}

pub(crate) use nop576;

macro_rules! nop577 {
    () => {{
        use core::arch::asm;
        nop288!();
        nop288!();
        asm!("nop");
    }};
}

pub(crate) use nop577;

macro_rules! nop578 {
    () => {{
        use core::arch::asm;
        nop289!();
        nop289!();
    }};
}

pub(crate) use nop578;

macro_rules! nop579 {
    () => {{
        use core::arch::asm;
        nop289!();
        nop289!();
        asm!("nop");
    }};
}

pub(crate) use nop579;

macro_rules! nop580 {
    () => {{
        use core::arch::asm;
        nop290!();
        nop290!();
    }};
}

pub(crate) use nop580;

macro_rules! nop581 {
    () => {{
        use core::arch::asm;
        nop290!();
        nop290!();
        asm!("nop");
    }};
}

pub(crate) use nop581;

macro_rules! nop582 {
    () => {{
        use core::arch::asm;
        nop291!();
        nop291!();
    }};
}

pub(crate) use nop582;

macro_rules! nop583 {
    () => {{
        use core::arch::asm;
        nop291!();
        nop291!();
        asm!("nop");
    }};
}

pub(crate) use nop583;

macro_rules! nop584 {
    () => {{
        use core::arch::asm;
        nop292!();
        nop292!();
    }};
}

pub(crate) use nop584;

macro_rules! nop585 {
    () => {{
        use core::arch::asm;
        nop292!();
        nop292!();
        asm!("nop");
    }};
}

pub(crate) use nop585;

macro_rules! nop586 {
    () => {{
        use core::arch::asm;
        nop293!();
        nop293!();
    }};
}

pub(crate) use nop586;

macro_rules! nop587 {
    () => {{
        use core::arch::asm;
        nop293!();
        nop293!();
        asm!("nop");
    }};
}

pub(crate) use nop587;

macro_rules! nop588 {
    () => {{
        use core::arch::asm;
        nop294!();
        nop294!();
    }};
}

pub(crate) use nop588;

macro_rules! nop589 {
    () => {{
        use core::arch::asm;
        nop294!();
        nop294!();
        asm!("nop");
    }};
}

pub(crate) use nop589;

macro_rules! nop590 {
    () => {{
        use core::arch::asm;
        nop295!();
        nop295!();
    }};
}

pub(crate) use nop590;

macro_rules! nop591 {
    () => {{
        use core::arch::asm;
        nop295!();
        nop295!();
        asm!("nop");
    }};
}

pub(crate) use nop591;

macro_rules! nop592 {
    () => {{
        use core::arch::asm;
        nop296!();
        nop296!();
    }};
}

pub(crate) use nop592;

macro_rules! nop593 {
    () => {{
        use core::arch::asm;
        nop296!();
        nop296!();
        asm!("nop");
    }};
}

pub(crate) use nop593;

macro_rules! nop594 {
    () => {{
        use core::arch::asm;
        nop297!();
        nop297!();
    }};
}

pub(crate) use nop594;

macro_rules! nop595 {
    () => {{
        use core::arch::asm;
        nop297!();
        nop297!();
        asm!("nop");
    }};
}

pub(crate) use nop595;

macro_rules! nop596 {
    () => {{
        use core::arch::asm;
        nop298!();
        nop298!();
    }};
}

pub(crate) use nop596;

macro_rules! nop597 {
    () => {{
        use core::arch::asm;
        nop298!();
        nop298!();
        asm!("nop");
    }};
}

pub(crate) use nop597;

macro_rules! nop598 {
    () => {{
        use core::arch::asm;
        nop299!();
        nop299!();
    }};
}

pub(crate) use nop598;

macro_rules! nop599 {
    () => {{
        use core::arch::asm;
        nop299!();
        nop299!();
        asm!("nop");
    }};
}

pub(crate) use nop599;

macro_rules! nop600 {
    () => {{
        use core::arch::asm;
        nop300!();
        nop300!();
    }};
}

pub(crate) use nop600;

macro_rules! nop601 {
    () => {{
        use core::arch::asm;
        nop300!();
        nop300!();
        asm!("nop");
    }};
}

pub(crate) use nop601;

macro_rules! nop602 {
    () => {{
        use core::arch::asm;
        nop301!();
        nop301!();
    }};
}

pub(crate) use nop602;

macro_rules! nop603 {
    () => {{
        use core::arch::asm;
        nop301!();
        nop301!();
        asm!("nop");
    }};
}

pub(crate) use nop603;

macro_rules! nop604 {
    () => {{
        use core::arch::asm;
        nop302!();
        nop302!();
    }};
}

pub(crate) use nop604;

macro_rules! nop605 {
    () => {{
        use core::arch::asm;
        nop302!();
        nop302!();
        asm!("nop");
    }};
}

pub(crate) use nop605;

macro_rules! nop606 {
    () => {{
        use core::arch::asm;
        nop303!();
        nop303!();
    }};
}

pub(crate) use nop606;

macro_rules! nop607 {
    () => {{
        use core::arch::asm;
        nop303!();
        nop303!();
        asm!("nop");
    }};
}

pub(crate) use nop607;

macro_rules! nop608 {
    () => {{
        use core::arch::asm;
        nop304!();
        nop304!();
    }};
}

pub(crate) use nop608;

macro_rules! nop609 {
    () => {{
        use core::arch::asm;
        nop304!();
        nop304!();
        asm!("nop");
    }};
}

pub(crate) use nop609;

macro_rules! nop610 {
    () => {{
        use core::arch::asm;
        nop305!();
        nop305!();
    }};
}

pub(crate) use nop610;

macro_rules! nop611 {
    () => {{
        use core::arch::asm;
        nop305!();
        nop305!();
        asm!("nop");
    }};
}

pub(crate) use nop611;

macro_rules! nop612 {
    () => {{
        use core::arch::asm;
        nop306!();
        nop306!();
    }};
}

pub(crate) use nop612;

macro_rules! nop613 {
    () => {{
        use core::arch::asm;
        nop306!();
        nop306!();
        asm!("nop");
    }};
}

pub(crate) use nop613;

macro_rules! nop614 {
    () => {{
        use core::arch::asm;
        nop307!();
        nop307!();
    }};
}

pub(crate) use nop614;

macro_rules! nop615 {
    () => {{
        use core::arch::asm;
        nop307!();
        nop307!();
        asm!("nop");
    }};
}

pub(crate) use nop615;

macro_rules! nop616 {
    () => {{
        use core::arch::asm;
        nop308!();
        nop308!();
    }};
}

pub(crate) use nop616;

macro_rules! nop617 {
    () => {{
        use core::arch::asm;
        nop308!();
        nop308!();
        asm!("nop");
    }};
}

pub(crate) use nop617;

macro_rules! nop618 {
    () => {{
        use core::arch::asm;
        nop309!();
        nop309!();
    }};
}

pub(crate) use nop618;

macro_rules! nop619 {
    () => {{
        use core::arch::asm;
        nop309!();
        nop309!();
        asm!("nop");
    }};
}

pub(crate) use nop619;

macro_rules! nop620 {
    () => {{
        use core::arch::asm;
        nop310!();
        nop310!();
    }};
}

pub(crate) use nop620;

macro_rules! nop621 {
    () => {{
        use core::arch::asm;
        nop310!();
        nop310!();
        asm!("nop");
    }};
}

pub(crate) use nop621;

macro_rules! nop622 {
    () => {{
        use core::arch::asm;
        nop311!();
        nop311!();
    }};
}

pub(crate) use nop622;

macro_rules! nop623 {
    () => {{
        use core::arch::asm;
        nop311!();
        nop311!();
        asm!("nop");
    }};
}

pub(crate) use nop623;

macro_rules! nop624 {
    () => {{
        use core::arch::asm;
        nop312!();
        nop312!();
    }};
}

pub(crate) use nop624;

macro_rules! nop625 {
    () => {{
        use core::arch::asm;
        nop312!();
        nop312!();
        asm!("nop");
    }};
}

pub(crate) use nop625;

macro_rules! nop626 {
    () => {{
        use core::arch::asm;
        nop313!();
        nop313!();
    }};
}

pub(crate) use nop626;

macro_rules! nop627 {
    () => {{
        use core::arch::asm;
        nop313!();
        nop313!();
        asm!("nop");
    }};
}

pub(crate) use nop627;

macro_rules! nop628 {
    () => {{
        use core::arch::asm;
        nop314!();
        nop314!();
    }};
}

pub(crate) use nop628;

macro_rules! nop629 {
    () => {{
        use core::arch::asm;
        nop314!();
        nop314!();
        asm!("nop");
    }};
}

pub(crate) use nop629;

macro_rules! nop630 {
    () => {{
        use core::arch::asm;
        nop315!();
        nop315!();
    }};
}

pub(crate) use nop630;

macro_rules! nop631 {
    () => {{
        use core::arch::asm;
        nop315!();
        nop315!();
        asm!("nop");
    }};
}

pub(crate) use nop631;

macro_rules! nop632 {
    () => {{
        use core::arch::asm;
        nop316!();
        nop316!();
    }};
}

pub(crate) use nop632;

macro_rules! nop633 {
    () => {{
        use core::arch::asm;
        nop316!();
        nop316!();
        asm!("nop");
    }};
}

pub(crate) use nop633;

macro_rules! nop634 {
    () => {{
        use core::arch::asm;
        nop317!();
        nop317!();
    }};
}

pub(crate) use nop634;

macro_rules! nop635 {
    () => {{
        use core::arch::asm;
        nop317!();
        nop317!();
        asm!("nop");
    }};
}

pub(crate) use nop635;

macro_rules! nop636 {
    () => {{
        use core::arch::asm;
        nop318!();
        nop318!();
    }};
}

pub(crate) use nop636;

macro_rules! nop637 {
    () => {{
        use core::arch::asm;
        nop318!();
        nop318!();
        asm!("nop");
    }};
}

pub(crate) use nop637;

macro_rules! nop638 {
    () => {{
        use core::arch::asm;
        nop319!();
        nop319!();
    }};
}

pub(crate) use nop638;

macro_rules! nop639 {
    () => {{
        use core::arch::asm;
        nop319!();
        nop319!();
        asm!("nop");
    }};
}

pub(crate) use nop639;

macro_rules! nop640 {
    () => {{
        use core::arch::asm;
        nop320!();
        nop320!();
    }};
}

pub(crate) use nop640;

macro_rules! nop641 {
    () => {{
        use core::arch::asm;
        nop320!();
        nop320!();
        asm!("nop");
    }};
}

pub(crate) use nop641;

macro_rules! nop642 {
    () => {{
        use core::arch::asm;
        nop321!();
        nop321!();
    }};
}

pub(crate) use nop642;

macro_rules! nop643 {
    () => {{
        use core::arch::asm;
        nop321!();
        nop321!();
        asm!("nop");
    }};
}

pub(crate) use nop643;

macro_rules! nop644 {
    () => {{
        use core::arch::asm;
        nop322!();
        nop322!();
    }};
}

pub(crate) use nop644;

macro_rules! nop645 {
    () => {{
        use core::arch::asm;
        nop322!();
        nop322!();
        asm!("nop");
    }};
}

pub(crate) use nop645;

macro_rules! nop646 {
    () => {{
        use core::arch::asm;
        nop323!();
        nop323!();
    }};
}

pub(crate) use nop646;

macro_rules! nop647 {
    () => {{
        use core::arch::asm;
        nop323!();
        nop323!();
        asm!("nop");
    }};
}

pub(crate) use nop647;

macro_rules! nop648 {
    () => {{
        use core::arch::asm;
        nop324!();
        nop324!();
    }};
}

pub(crate) use nop648;

macro_rules! nop649 {
    () => {{
        use core::arch::asm;
        nop324!();
        nop324!();
        asm!("nop");
    }};
}

pub(crate) use nop649;

macro_rules! nop650 {
    () => {{
        use core::arch::asm;
        nop325!();
        nop325!();
    }};
}

pub(crate) use nop650;

macro_rules! nop651 {
    () => {{
        use core::arch::asm;
        nop325!();
        nop325!();
        asm!("nop");
    }};
}

pub(crate) use nop651;

macro_rules! nop652 {
    () => {{
        use core::arch::asm;
        nop326!();
        nop326!();
    }};
}

pub(crate) use nop652;

macro_rules! nop653 {
    () => {{
        use core::arch::asm;
        nop326!();
        nop326!();
        asm!("nop");
    }};
}

pub(crate) use nop653;

macro_rules! nop654 {
    () => {{
        use core::arch::asm;
        nop327!();
        nop327!();
    }};
}

pub(crate) use nop654;

macro_rules! nop655 {
    () => {{
        use core::arch::asm;
        nop327!();
        nop327!();
        asm!("nop");
    }};
}

pub(crate) use nop655;

macro_rules! nop656 {
    () => {{
        use core::arch::asm;
        nop328!();
        nop328!();
    }};
}

pub(crate) use nop656;

macro_rules! nop657 {
    () => {{
        use core::arch::asm;
        nop328!();
        nop328!();
        asm!("nop");
    }};
}

pub(crate) use nop657;

macro_rules! nop658 {
    () => {{
        use core::arch::asm;
        nop329!();
        nop329!();
    }};
}

pub(crate) use nop658;

macro_rules! nop659 {
    () => {{
        use core::arch::asm;
        nop329!();
        nop329!();
        asm!("nop");
    }};
}

pub(crate) use nop659;

macro_rules! nop660 {
    () => {{
        use core::arch::asm;
        nop330!();
        nop330!();
    }};
}

pub(crate) use nop660;

macro_rules! nop661 {
    () => {{
        use core::arch::asm;
        nop330!();
        nop330!();
        asm!("nop");
    }};
}

pub(crate) use nop661;

macro_rules! nop662 {
    () => {{
        use core::arch::asm;
        nop331!();
        nop331!();
    }};
}

pub(crate) use nop662;

macro_rules! nop663 {
    () => {{
        use core::arch::asm;
        nop331!();
        nop331!();
        asm!("nop");
    }};
}

pub(crate) use nop663;

macro_rules! nop664 {
    () => {{
        use core::arch::asm;
        nop332!();
        nop332!();
    }};
}

pub(crate) use nop664;

macro_rules! nop665 {
    () => {{
        use core::arch::asm;
        nop332!();
        nop332!();
        asm!("nop");
    }};
}

pub(crate) use nop665;

macro_rules! nop666 {
    () => {{
        use core::arch::asm;
        nop333!();
        nop333!();
    }};
}

pub(crate) use nop666;

macro_rules! nop667 {
    () => {{
        use core::arch::asm;
        nop333!();
        nop333!();
        asm!("nop");
    }};
}

pub(crate) use nop667;

macro_rules! nop668 {
    () => {{
        use core::arch::asm;
        nop334!();
        nop334!();
    }};
}

pub(crate) use nop668;

macro_rules! nop669 {
    () => {{
        use core::arch::asm;
        nop334!();
        nop334!();
        asm!("nop");
    }};
}

pub(crate) use nop669;

macro_rules! nop670 {
    () => {{
        use core::arch::asm;
        nop335!();
        nop335!();
    }};
}

pub(crate) use nop670;

macro_rules! nop671 {
    () => {{
        use core::arch::asm;
        nop335!();
        nop335!();
        asm!("nop");
    }};
}

pub(crate) use nop671;

macro_rules! nop672 {
    () => {{
        use core::arch::asm;
        nop336!();
        nop336!();
    }};
}

pub(crate) use nop672;

macro_rules! nop673 {
    () => {{
        use core::arch::asm;
        nop336!();
        nop336!();
        asm!("nop");
    }};
}

pub(crate) use nop673;

macro_rules! nop674 {
    () => {{
        use core::arch::asm;
        nop337!();
        nop337!();
    }};
}

pub(crate) use nop674;

macro_rules! nop675 {
    () => {{
        use core::arch::asm;
        nop337!();
        nop337!();
        asm!("nop");
    }};
}

pub(crate) use nop675;

macro_rules! nop676 {
    () => {{
        use core::arch::asm;
        nop338!();
        nop338!();
    }};
}

pub(crate) use nop676;

macro_rules! nop677 {
    () => {{
        use core::arch::asm;
        nop338!();
        nop338!();
        asm!("nop");
    }};
}

pub(crate) use nop677;

macro_rules! nop678 {
    () => {{
        use core::arch::asm;
        nop339!();
        nop339!();
    }};
}

pub(crate) use nop678;

macro_rules! nop679 {
    () => {{
        use core::arch::asm;
        nop339!();
        nop339!();
        asm!("nop");
    }};
}

pub(crate) use nop679;

macro_rules! nop680 {
    () => {{
        use core::arch::asm;
        nop340!();
        nop340!();
    }};
}

pub(crate) use nop680;

macro_rules! nop681 {
    () => {{
        use core::arch::asm;
        nop340!();
        nop340!();
        asm!("nop");
    }};
}

pub(crate) use nop681;

macro_rules! nop682 {
    () => {{
        use core::arch::asm;
        nop341!();
        nop341!();
    }};
}

pub(crate) use nop682;

macro_rules! nop683 {
    () => {{
        use core::arch::asm;
        nop341!();
        nop341!();
        asm!("nop");
    }};
}

pub(crate) use nop683;

macro_rules! nop684 {
    () => {{
        use core::arch::asm;
        nop342!();
        nop342!();
    }};
}

pub(crate) use nop684;

macro_rules! nop685 {
    () => {{
        use core::arch::asm;
        nop342!();
        nop342!();
        asm!("nop");
    }};
}

pub(crate) use nop685;

macro_rules! nop686 {
    () => {{
        use core::arch::asm;
        nop343!();
        nop343!();
    }};
}

pub(crate) use nop686;

macro_rules! nop687 {
    () => {{
        use core::arch::asm;
        nop343!();
        nop343!();
        asm!("nop");
    }};
}

pub(crate) use nop687;

macro_rules! nop688 {
    () => {{
        use core::arch::asm;
        nop344!();
        nop344!();
    }};
}

pub(crate) use nop688;

macro_rules! nop689 {
    () => {{
        use core::arch::asm;
        nop344!();
        nop344!();
        asm!("nop");
    }};
}

pub(crate) use nop689;

macro_rules! nop690 {
    () => {{
        use core::arch::asm;
        nop345!();
        nop345!();
    }};
}

pub(crate) use nop690;

macro_rules! nop691 {
    () => {{
        use core::arch::asm;
        nop345!();
        nop345!();
        asm!("nop");
    }};
}

pub(crate) use nop691;

macro_rules! nop692 {
    () => {{
        use core::arch::asm;
        nop346!();
        nop346!();
    }};
}

pub(crate) use nop692;

macro_rules! nop693 {
    () => {{
        use core::arch::asm;
        nop346!();
        nop346!();
        asm!("nop");
    }};
}

pub(crate) use nop693;

macro_rules! nop694 {
    () => {{
        use core::arch::asm;
        nop347!();
        nop347!();
    }};
}

pub(crate) use nop694;

macro_rules! nop695 {
    () => {{
        use core::arch::asm;
        nop347!();
        nop347!();
        asm!("nop");
    }};
}

pub(crate) use nop695;

macro_rules! nop696 {
    () => {{
        use core::arch::asm;
        nop348!();
        nop348!();
    }};
}

pub(crate) use nop696;

macro_rules! nop697 {
    () => {{
        use core::arch::asm;
        nop348!();
        nop348!();
        asm!("nop");
    }};
}

pub(crate) use nop697;

macro_rules! nop698 {
    () => {{
        use core::arch::asm;
        nop349!();
        nop349!();
    }};
}

pub(crate) use nop698;

macro_rules! nop699 {
    () => {{
        use core::arch::asm;
        nop349!();
        nop349!();
        asm!("nop");
    }};
}

pub(crate) use nop699;

macro_rules! nop700 {
    () => {{
        use core::arch::asm;
        nop350!();
        nop350!();
    }};
}

pub(crate) use nop700;

macro_rules! nop701 {
    () => {{
        use core::arch::asm;
        nop350!();
        nop350!();
        asm!("nop");
    }};
}

pub(crate) use nop701;

macro_rules! nop702 {
    () => {{
        use core::arch::asm;
        nop351!();
        nop351!();
    }};
}

pub(crate) use nop702;

macro_rules! nop703 {
    () => {{
        use core::arch::asm;
        nop351!();
        nop351!();
        asm!("nop");
    }};
}

pub(crate) use nop703;

macro_rules! nop704 {
    () => {{
        use core::arch::asm;
        nop352!();
        nop352!();
    }};
}

pub(crate) use nop704;

macro_rules! nop705 {
    () => {{
        use core::arch::asm;
        nop352!();
        nop352!();
        asm!("nop");
    }};
}

pub(crate) use nop705;

macro_rules! nop706 {
    () => {{
        use core::arch::asm;
        nop353!();
        nop353!();
    }};
}

pub(crate) use nop706;

macro_rules! nop707 {
    () => {{
        use core::arch::asm;
        nop353!();
        nop353!();
        asm!("nop");
    }};
}

pub(crate) use nop707;

macro_rules! nop708 {
    () => {{
        use core::arch::asm;
        nop354!();
        nop354!();
    }};
}

pub(crate) use nop708;

macro_rules! nop709 {
    () => {{
        use core::arch::asm;
        nop354!();
        nop354!();
        asm!("nop");
    }};
}

pub(crate) use nop709;

macro_rules! nop710 {
    () => {{
        use core::arch::asm;
        nop355!();
        nop355!();
    }};
}

pub(crate) use nop710;

macro_rules! nop711 {
    () => {{
        use core::arch::asm;
        nop355!();
        nop355!();
        asm!("nop");
    }};
}

pub(crate) use nop711;

macro_rules! nop712 {
    () => {{
        use core::arch::asm;
        nop356!();
        nop356!();
    }};
}

pub(crate) use nop712;

macro_rules! nop713 {
    () => {{
        use core::arch::asm;
        nop356!();
        nop356!();
        asm!("nop");
    }};
}

pub(crate) use nop713;

macro_rules! nop714 {
    () => {{
        use core::arch::asm;
        nop357!();
        nop357!();
    }};
}

pub(crate) use nop714;

macro_rules! nop715 {
    () => {{
        use core::arch::asm;
        nop357!();
        nop357!();
        asm!("nop");
    }};
}

pub(crate) use nop715;

macro_rules! nop716 {
    () => {{
        use core::arch::asm;
        nop358!();
        nop358!();
    }};
}

pub(crate) use nop716;

macro_rules! nop717 {
    () => {{
        use core::arch::asm;
        nop358!();
        nop358!();
        asm!("nop");
    }};
}

pub(crate) use nop717;

macro_rules! nop718 {
    () => {{
        use core::arch::asm;
        nop359!();
        nop359!();
    }};
}

pub(crate) use nop718;

macro_rules! nop719 {
    () => {{
        use core::arch::asm;
        nop359!();
        nop359!();
        asm!("nop");
    }};
}

pub(crate) use nop719;

macro_rules! nop720 {
    () => {{
        use core::arch::asm;
        nop360!();
        nop360!();
    }};
}

pub(crate) use nop720;

macro_rules! nop721 {
    () => {{
        use core::arch::asm;
        nop360!();
        nop360!();
        asm!("nop");
    }};
}

pub(crate) use nop721;

macro_rules! nop722 {
    () => {{
        use core::arch::asm;
        nop361!();
        nop361!();
    }};
}

pub(crate) use nop722;

macro_rules! nop723 {
    () => {{
        use core::arch::asm;
        nop361!();
        nop361!();
        asm!("nop");
    }};
}

pub(crate) use nop723;

macro_rules! nop724 {
    () => {{
        use core::arch::asm;
        nop362!();
        nop362!();
    }};
}

pub(crate) use nop724;

macro_rules! nop725 {
    () => {{
        use core::arch::asm;
        nop362!();
        nop362!();
        asm!("nop");
    }};
}

pub(crate) use nop725;

macro_rules! nop726 {
    () => {{
        use core::arch::asm;
        nop363!();
        nop363!();
    }};
}

pub(crate) use nop726;

macro_rules! nop727 {
    () => {{
        use core::arch::asm;
        nop363!();
        nop363!();
        asm!("nop");
    }};
}

pub(crate) use nop727;

macro_rules! nop728 {
    () => {{
        use core::arch::asm;
        nop364!();
        nop364!();
    }};
}

pub(crate) use nop728;

macro_rules! nop729 {
    () => {{
        use core::arch::asm;
        nop364!();
        nop364!();
        asm!("nop");
    }};
}

pub(crate) use nop729;

macro_rules! nop730 {
    () => {{
        use core::arch::asm;
        nop365!();
        nop365!();
    }};
}

pub(crate) use nop730;

macro_rules! nop731 {
    () => {{
        use core::arch::asm;
        nop365!();
        nop365!();
        asm!("nop");
    }};
}

pub(crate) use nop731;

macro_rules! nop732 {
    () => {{
        use core::arch::asm;
        nop366!();
        nop366!();
    }};
}

pub(crate) use nop732;

macro_rules! nop733 {
    () => {{
        use core::arch::asm;
        nop366!();
        nop366!();
        asm!("nop");
    }};
}

pub(crate) use nop733;

macro_rules! nop734 {
    () => {{
        use core::arch::asm;
        nop367!();
        nop367!();
    }};
}

pub(crate) use nop734;

macro_rules! nop735 {
    () => {{
        use core::arch::asm;
        nop367!();
        nop367!();
        asm!("nop");
    }};
}

pub(crate) use nop735;

macro_rules! nop736 {
    () => {{
        use core::arch::asm;
        nop368!();
        nop368!();
    }};
}

pub(crate) use nop736;

macro_rules! nop737 {
    () => {{
        use core::arch::asm;
        nop368!();
        nop368!();
        asm!("nop");
    }};
}

pub(crate) use nop737;

macro_rules! nop738 {
    () => {{
        use core::arch::asm;
        nop369!();
        nop369!();
    }};
}

pub(crate) use nop738;

macro_rules! nop739 {
    () => {{
        use core::arch::asm;
        nop369!();
        nop369!();
        asm!("nop");
    }};
}

pub(crate) use nop739;

macro_rules! nop740 {
    () => {{
        use core::arch::asm;
        nop370!();
        nop370!();
    }};
}

pub(crate) use nop740;

macro_rules! nop741 {
    () => {{
        use core::arch::asm;
        nop370!();
        nop370!();
        asm!("nop");
    }};
}

pub(crate) use nop741;

macro_rules! nop742 {
    () => {{
        use core::arch::asm;
        nop371!();
        nop371!();
    }};
}

pub(crate) use nop742;

macro_rules! nop743 {
    () => {{
        use core::arch::asm;
        nop371!();
        nop371!();
        asm!("nop");
    }};
}

pub(crate) use nop743;

macro_rules! nop744 {
    () => {{
        use core::arch::asm;
        nop372!();
        nop372!();
    }};
}

pub(crate) use nop744;

macro_rules! nop745 {
    () => {{
        use core::arch::asm;
        nop372!();
        nop372!();
        asm!("nop");
    }};
}

pub(crate) use nop745;

macro_rules! nop746 {
    () => {{
        use core::arch::asm;
        nop373!();
        nop373!();
    }};
}

pub(crate) use nop746;

macro_rules! nop747 {
    () => {{
        use core::arch::asm;
        nop373!();
        nop373!();
        asm!("nop");
    }};
}

pub(crate) use nop747;

macro_rules! nop748 {
    () => {{
        use core::arch::asm;
        nop374!();
        nop374!();
    }};
}

pub(crate) use nop748;

macro_rules! nop749 {
    () => {{
        use core::arch::asm;
        nop374!();
        nop374!();
        asm!("nop");
    }};
}

pub(crate) use nop749;

macro_rules! nop750 {
    () => {{
        use core::arch::asm;
        nop375!();
        nop375!();
    }};
}

pub(crate) use nop750;

macro_rules! nop751 {
    () => {{
        use core::arch::asm;
        nop375!();
        nop375!();
        asm!("nop");
    }};
}

pub(crate) use nop751;

macro_rules! nop752 {
    () => {{
        use core::arch::asm;
        nop376!();
        nop376!();
    }};
}

pub(crate) use nop752;

macro_rules! nop753 {
    () => {{
        use core::arch::asm;
        nop376!();
        nop376!();
        asm!("nop");
    }};
}

pub(crate) use nop753;

macro_rules! nop754 {
    () => {{
        use core::arch::asm;
        nop377!();
        nop377!();
    }};
}

pub(crate) use nop754;

macro_rules! nop755 {
    () => {{
        use core::arch::asm;
        nop377!();
        nop377!();
        asm!("nop");
    }};
}

pub(crate) use nop755;

macro_rules! nop756 {
    () => {{
        use core::arch::asm;
        nop378!();
        nop378!();
    }};
}

pub(crate) use nop756;

macro_rules! nop757 {
    () => {{
        use core::arch::asm;
        nop378!();
        nop378!();
        asm!("nop");
    }};
}

pub(crate) use nop757;

macro_rules! nop758 {
    () => {{
        use core::arch::asm;
        nop379!();
        nop379!();
    }};
}

pub(crate) use nop758;

macro_rules! nop759 {
    () => {{
        use core::arch::asm;
        nop379!();
        nop379!();
        asm!("nop");
    }};
}

pub(crate) use nop759;

macro_rules! nop760 {
    () => {{
        use core::arch::asm;
        nop380!();
        nop380!();
    }};
}

pub(crate) use nop760;

macro_rules! nop761 {
    () => {{
        use core::arch::asm;
        nop380!();
        nop380!();
        asm!("nop");
    }};
}

pub(crate) use nop761;

macro_rules! nop762 {
    () => {{
        use core::arch::asm;
        nop381!();
        nop381!();
    }};
}

pub(crate) use nop762;

macro_rules! nop763 {
    () => {{
        use core::arch::asm;
        nop381!();
        nop381!();
        asm!("nop");
    }};
}

pub(crate) use nop763;

macro_rules! nop764 {
    () => {{
        use core::arch::asm;
        nop382!();
        nop382!();
    }};
}

pub(crate) use nop764;

macro_rules! nop765 {
    () => {{
        use core::arch::asm;
        nop382!();
        nop382!();
        asm!("nop");
    }};
}

pub(crate) use nop765;

macro_rules! nop766 {
    () => {{
        use core::arch::asm;
        nop383!();
        nop383!();
    }};
}

pub(crate) use nop766;

macro_rules! nop767 {
    () => {{
        use core::arch::asm;
        nop383!();
        nop383!();
        asm!("nop");
    }};
}

pub(crate) use nop767;

macro_rules! nop768 {
    () => {{
        use core::arch::asm;
        nop384!();
        nop384!();
    }};
}

pub(crate) use nop768;

macro_rules! nop769 {
    () => {{
        use core::arch::asm;
        nop384!();
        nop384!();
        asm!("nop");
    }};
}

pub(crate) use nop769;

macro_rules! nop770 {
    () => {{
        use core::arch::asm;
        nop385!();
        nop385!();
    }};
}

pub(crate) use nop770;

macro_rules! nop771 {
    () => {{
        use core::arch::asm;
        nop385!();
        nop385!();
        asm!("nop");
    }};
}

pub(crate) use nop771;

macro_rules! nop772 {
    () => {{
        use core::arch::asm;
        nop386!();
        nop386!();
    }};
}

pub(crate) use nop772;

macro_rules! nop773 {
    () => {{
        use core::arch::asm;
        nop386!();
        nop386!();
        asm!("nop");
    }};
}

pub(crate) use nop773;

macro_rules! nop774 {
    () => {{
        use core::arch::asm;
        nop387!();
        nop387!();
    }};
}

pub(crate) use nop774;

macro_rules! nop775 {
    () => {{
        use core::arch::asm;
        nop387!();
        nop387!();
        asm!("nop");
    }};
}

pub(crate) use nop775;

macro_rules! nop776 {
    () => {{
        use core::arch::asm;
        nop388!();
        nop388!();
    }};
}

pub(crate) use nop776;

macro_rules! nop777 {
    () => {{
        use core::arch::asm;
        nop388!();
        nop388!();
        asm!("nop");
    }};
}

pub(crate) use nop777;

macro_rules! nop778 {
    () => {{
        use core::arch::asm;
        nop389!();
        nop389!();
    }};
}

pub(crate) use nop778;

macro_rules! nop779 {
    () => {{
        use core::arch::asm;
        nop389!();
        nop389!();
        asm!("nop");
    }};
}

pub(crate) use nop779;

macro_rules! nop780 {
    () => {{
        use core::arch::asm;
        nop390!();
        nop390!();
    }};
}

pub(crate) use nop780;

macro_rules! nop781 {
    () => {{
        use core::arch::asm;
        nop390!();
        nop390!();
        asm!("nop");
    }};
}

pub(crate) use nop781;

macro_rules! nop782 {
    () => {{
        use core::arch::asm;
        nop391!();
        nop391!();
    }};
}

pub(crate) use nop782;

macro_rules! nop783 {
    () => {{
        use core::arch::asm;
        nop391!();
        nop391!();
        asm!("nop");
    }};
}

pub(crate) use nop783;

macro_rules! nop784 {
    () => {{
        use core::arch::asm;
        nop392!();
        nop392!();
    }};
}

pub(crate) use nop784;

macro_rules! nop785 {
    () => {{
        use core::arch::asm;
        nop392!();
        nop392!();
        asm!("nop");
    }};
}

pub(crate) use nop785;

macro_rules! nop786 {
    () => {{
        use core::arch::asm;
        nop393!();
        nop393!();
    }};
}

pub(crate) use nop786;

macro_rules! nop787 {
    () => {{
        use core::arch::asm;
        nop393!();
        nop393!();
        asm!("nop");
    }};
}

pub(crate) use nop787;

macro_rules! nop788 {
    () => {{
        use core::arch::asm;
        nop394!();
        nop394!();
    }};
}

pub(crate) use nop788;

macro_rules! nop789 {
    () => {{
        use core::arch::asm;
        nop394!();
        nop394!();
        asm!("nop");
    }};
}

pub(crate) use nop789;

macro_rules! nop790 {
    () => {{
        use core::arch::asm;
        nop395!();
        nop395!();
    }};
}

pub(crate) use nop790;

macro_rules! nop791 {
    () => {{
        use core::arch::asm;
        nop395!();
        nop395!();
        asm!("nop");
    }};
}

pub(crate) use nop791;

macro_rules! nop792 {
    () => {{
        use core::arch::asm;
        nop396!();
        nop396!();
    }};
}

pub(crate) use nop792;

macro_rules! nop793 {
    () => {{
        use core::arch::asm;
        nop396!();
        nop396!();
        asm!("nop");
    }};
}

pub(crate) use nop793;

macro_rules! nop794 {
    () => {{
        use core::arch::asm;
        nop397!();
        nop397!();
    }};
}

pub(crate) use nop794;

macro_rules! nop795 {
    () => {{
        use core::arch::asm;
        nop397!();
        nop397!();
        asm!("nop");
    }};
}

pub(crate) use nop795;

macro_rules! nop796 {
    () => {{
        use core::arch::asm;
        nop398!();
        nop398!();
    }};
}

pub(crate) use nop796;

macro_rules! nop797 {
    () => {{
        use core::arch::asm;
        nop398!();
        nop398!();
        asm!("nop");
    }};
}

pub(crate) use nop797;

macro_rules! nop798 {
    () => {{
        use core::arch::asm;
        nop399!();
        nop399!();
    }};
}

pub(crate) use nop798;

macro_rules! nop799 {
    () => {{
        use core::arch::asm;
        nop399!();
        nop399!();
        asm!("nop");
    }};
}

pub(crate) use nop799;

macro_rules! nop800 {
    () => {{
        use core::arch::asm;
        nop400!();
        nop400!();
    }};
}

pub(crate) use nop800;

macro_rules! nop801 {
    () => {{
        use core::arch::asm;
        nop400!();
        nop400!();
        asm!("nop");
    }};
}

pub(crate) use nop801;

macro_rules! nop802 {
    () => {{
        use core::arch::asm;
        nop401!();
        nop401!();
    }};
}

pub(crate) use nop802;

macro_rules! nop803 {
    () => {{
        use core::arch::asm;
        nop401!();
        nop401!();
        asm!("nop");
    }};
}

pub(crate) use nop803;

macro_rules! nop804 {
    () => {{
        use core::arch::asm;
        nop402!();
        nop402!();
    }};
}

pub(crate) use nop804;

macro_rules! nop805 {
    () => {{
        use core::arch::asm;
        nop402!();
        nop402!();
        asm!("nop");
    }};
}

pub(crate) use nop805;

macro_rules! nop806 {
    () => {{
        use core::arch::asm;
        nop403!();
        nop403!();
    }};
}

pub(crate) use nop806;

macro_rules! nop807 {
    () => {{
        use core::arch::asm;
        nop403!();
        nop403!();
        asm!("nop");
    }};
}

pub(crate) use nop807;

macro_rules! nop808 {
    () => {{
        use core::arch::asm;
        nop404!();
        nop404!();
    }};
}

pub(crate) use nop808;

macro_rules! nop809 {
    () => {{
        use core::arch::asm;
        nop404!();
        nop404!();
        asm!("nop");
    }};
}

pub(crate) use nop809;

macro_rules! nop810 {
    () => {{
        use core::arch::asm;
        nop405!();
        nop405!();
    }};
}

pub(crate) use nop810;

macro_rules! nop811 {
    () => {{
        use core::arch::asm;
        nop405!();
        nop405!();
        asm!("nop");
    }};
}

pub(crate) use nop811;

macro_rules! nop812 {
    () => {{
        use core::arch::asm;
        nop406!();
        nop406!();
    }};
}

pub(crate) use nop812;

macro_rules! nop813 {
    () => {{
        use core::arch::asm;
        nop406!();
        nop406!();
        asm!("nop");
    }};
}

pub(crate) use nop813;

macro_rules! nop814 {
    () => {{
        use core::arch::asm;
        nop407!();
        nop407!();
    }};
}

pub(crate) use nop814;

macro_rules! nop815 {
    () => {{
        use core::arch::asm;
        nop407!();
        nop407!();
        asm!("nop");
    }};
}

pub(crate) use nop815;

macro_rules! nop816 {
    () => {{
        use core::arch::asm;
        nop408!();
        nop408!();
    }};
}

pub(crate) use nop816;

macro_rules! nop817 {
    () => {{
        use core::arch::asm;
        nop408!();
        nop408!();
        asm!("nop");
    }};
}

pub(crate) use nop817;

macro_rules! nop818 {
    () => {{
        use core::arch::asm;
        nop409!();
        nop409!();
    }};
}

pub(crate) use nop818;

macro_rules! nop819 {
    () => {{
        use core::arch::asm;
        nop409!();
        nop409!();
        asm!("nop");
    }};
}

pub(crate) use nop819;

macro_rules! nop820 {
    () => {{
        use core::arch::asm;
        nop410!();
        nop410!();
    }};
}

pub(crate) use nop820;

macro_rules! nop821 {
    () => {{
        use core::arch::asm;
        nop410!();
        nop410!();
        asm!("nop");
    }};
}

pub(crate) use nop821;

macro_rules! nop822 {
    () => {{
        use core::arch::asm;
        nop411!();
        nop411!();
    }};
}

pub(crate) use nop822;

macro_rules! nop823 {
    () => {{
        use core::arch::asm;
        nop411!();
        nop411!();
        asm!("nop");
    }};
}

pub(crate) use nop823;

macro_rules! nop824 {
    () => {{
        use core::arch::asm;
        nop412!();
        nop412!();
    }};
}

pub(crate) use nop824;

macro_rules! nop825 {
    () => {{
        use core::arch::asm;
        nop412!();
        nop412!();
        asm!("nop");
    }};
}

pub(crate) use nop825;

macro_rules! nop826 {
    () => {{
        use core::arch::asm;
        nop413!();
        nop413!();
    }};
}

pub(crate) use nop826;

macro_rules! nop827 {
    () => {{
        use core::arch::asm;
        nop413!();
        nop413!();
        asm!("nop");
    }};
}

pub(crate) use nop827;

macro_rules! nop828 {
    () => {{
        use core::arch::asm;
        nop414!();
        nop414!();
    }};
}

pub(crate) use nop828;

macro_rules! nop829 {
    () => {{
        use core::arch::asm;
        nop414!();
        nop414!();
        asm!("nop");
    }};
}

pub(crate) use nop829;

macro_rules! nop830 {
    () => {{
        use core::arch::asm;
        nop415!();
        nop415!();
    }};
}

pub(crate) use nop830;

macro_rules! nop831 {
    () => {{
        use core::arch::asm;
        nop415!();
        nop415!();
        asm!("nop");
    }};
}

pub(crate) use nop831;

macro_rules! nop832 {
    () => {{
        use core::arch::asm;
        nop416!();
        nop416!();
    }};
}

pub(crate) use nop832;

macro_rules! nop833 {
    () => {{
        use core::arch::asm;
        nop416!();
        nop416!();
        asm!("nop");
    }};
}

pub(crate) use nop833;

macro_rules! nop834 {
    () => {{
        use core::arch::asm;
        nop417!();
        nop417!();
    }};
}

pub(crate) use nop834;

macro_rules! nop835 {
    () => {{
        use core::arch::asm;
        nop417!();
        nop417!();
        asm!("nop");
    }};
}

pub(crate) use nop835;

macro_rules! nop836 {
    () => {{
        use core::arch::asm;
        nop418!();
        nop418!();
    }};
}

pub(crate) use nop836;

macro_rules! nop837 {
    () => {{
        use core::arch::asm;
        nop418!();
        nop418!();
        asm!("nop");
    }};
}

pub(crate) use nop837;

macro_rules! nop838 {
    () => {{
        use core::arch::asm;
        nop419!();
        nop419!();
    }};
}

pub(crate) use nop838;

macro_rules! nop839 {
    () => {{
        use core::arch::asm;
        nop419!();
        nop419!();
        asm!("nop");
    }};
}

pub(crate) use nop839;

macro_rules! nop840 {
    () => {{
        use core::arch::asm;
        nop420!();
        nop420!();
    }};
}

pub(crate) use nop840;

macro_rules! nop841 {
    () => {{
        use core::arch::asm;
        nop420!();
        nop420!();
        asm!("nop");
    }};
}

pub(crate) use nop841;

macro_rules! nop842 {
    () => {{
        use core::arch::asm;
        nop421!();
        nop421!();
    }};
}

pub(crate) use nop842;

macro_rules! nop843 {
    () => {{
        use core::arch::asm;
        nop421!();
        nop421!();
        asm!("nop");
    }};
}

pub(crate) use nop843;

macro_rules! nop844 {
    () => {{
        use core::arch::asm;
        nop422!();
        nop422!();
    }};
}

pub(crate) use nop844;

macro_rules! nop845 {
    () => {{
        use core::arch::asm;
        nop422!();
        nop422!();
        asm!("nop");
    }};
}

pub(crate) use nop845;

macro_rules! nop846 {
    () => {{
        use core::arch::asm;
        nop423!();
        nop423!();
    }};
}

pub(crate) use nop846;

macro_rules! nop847 {
    () => {{
        use core::arch::asm;
        nop423!();
        nop423!();
        asm!("nop");
    }};
}

pub(crate) use nop847;

macro_rules! nop848 {
    () => {{
        use core::arch::asm;
        nop424!();
        nop424!();
    }};
}

pub(crate) use nop848;

macro_rules! nop849 {
    () => {{
        use core::arch::asm;
        nop424!();
        nop424!();
        asm!("nop");
    }};
}

pub(crate) use nop849;

macro_rules! nop850 {
    () => {{
        use core::arch::asm;
        nop425!();
        nop425!();
    }};
}

pub(crate) use nop850;

macro_rules! nop851 {
    () => {{
        use core::arch::asm;
        nop425!();
        nop425!();
        asm!("nop");
    }};
}

pub(crate) use nop851;

macro_rules! nop852 {
    () => {{
        use core::arch::asm;
        nop426!();
        nop426!();
    }};
}

pub(crate) use nop852;

macro_rules! nop853 {
    () => {{
        use core::arch::asm;
        nop426!();
        nop426!();
        asm!("nop");
    }};
}

pub(crate) use nop853;

macro_rules! nop854 {
    () => {{
        use core::arch::asm;
        nop427!();
        nop427!();
    }};
}

pub(crate) use nop854;

macro_rules! nop855 {
    () => {{
        use core::arch::asm;
        nop427!();
        nop427!();
        asm!("nop");
    }};
}

pub(crate) use nop855;

macro_rules! nop856 {
    () => {{
        use core::arch::asm;
        nop428!();
        nop428!();
    }};
}

pub(crate) use nop856;

macro_rules! nop857 {
    () => {{
        use core::arch::asm;
        nop428!();
        nop428!();
        asm!("nop");
    }};
}

pub(crate) use nop857;

macro_rules! nop858 {
    () => {{
        use core::arch::asm;
        nop429!();
        nop429!();
    }};
}

pub(crate) use nop858;

macro_rules! nop859 {
    () => {{
        use core::arch::asm;
        nop429!();
        nop429!();
        asm!("nop");
    }};
}

pub(crate) use nop859;

macro_rules! nop860 {
    () => {{
        use core::arch::asm;
        nop430!();
        nop430!();
    }};
}

pub(crate) use nop860;

macro_rules! nop861 {
    () => {{
        use core::arch::asm;
        nop430!();
        nop430!();
        asm!("nop");
    }};
}

pub(crate) use nop861;

macro_rules! nop862 {
    () => {{
        use core::arch::asm;
        nop431!();
        nop431!();
    }};
}

pub(crate) use nop862;

macro_rules! nop863 {
    () => {{
        use core::arch::asm;
        nop431!();
        nop431!();
        asm!("nop");
    }};
}

pub(crate) use nop863;

macro_rules! nop864 {
    () => {{
        use core::arch::asm;
        nop432!();
        nop432!();
    }};
}

pub(crate) use nop864;

macro_rules! nop865 {
    () => {{
        use core::arch::asm;
        nop432!();
        nop432!();
        asm!("nop");
    }};
}

pub(crate) use nop865;

macro_rules! nop866 {
    () => {{
        use core::arch::asm;
        nop433!();
        nop433!();
    }};
}

pub(crate) use nop866;

macro_rules! nop867 {
    () => {{
        use core::arch::asm;
        nop433!();
        nop433!();
        asm!("nop");
    }};
}

pub(crate) use nop867;

macro_rules! nop868 {
    () => {{
        use core::arch::asm;
        nop434!();
        nop434!();
    }};
}

pub(crate) use nop868;

macro_rules! nop869 {
    () => {{
        use core::arch::asm;
        nop434!();
        nop434!();
        asm!("nop");
    }};
}

pub(crate) use nop869;

macro_rules! nop870 {
    () => {{
        use core::arch::asm;
        nop435!();
        nop435!();
    }};
}

pub(crate) use nop870;

macro_rules! nop871 {
    () => {{
        use core::arch::asm;
        nop435!();
        nop435!();
        asm!("nop");
    }};
}

pub(crate) use nop871;

macro_rules! nop872 {
    () => {{
        use core::arch::asm;
        nop436!();
        nop436!();
    }};
}

pub(crate) use nop872;

macro_rules! nop873 {
    () => {{
        use core::arch::asm;
        nop436!();
        nop436!();
        asm!("nop");
    }};
}

pub(crate) use nop873;

macro_rules! nop874 {
    () => {{
        use core::arch::asm;
        nop437!();
        nop437!();
    }};
}

pub(crate) use nop874;

macro_rules! nop875 {
    () => {{
        use core::arch::asm;
        nop437!();
        nop437!();
        asm!("nop");
    }};
}

pub(crate) use nop875;

macro_rules! nop876 {
    () => {{
        use core::arch::asm;
        nop438!();
        nop438!();
    }};
}

pub(crate) use nop876;

macro_rules! nop877 {
    () => {{
        use core::arch::asm;
        nop438!();
        nop438!();
        asm!("nop");
    }};
}

pub(crate) use nop877;

macro_rules! nop878 {
    () => {{
        use core::arch::asm;
        nop439!();
        nop439!();
    }};
}

pub(crate) use nop878;

macro_rules! nop879 {
    () => {{
        use core::arch::asm;
        nop439!();
        nop439!();
        asm!("nop");
    }};
}

pub(crate) use nop879;

macro_rules! nop880 {
    () => {{
        use core::arch::asm;
        nop440!();
        nop440!();
    }};
}

pub(crate) use nop880;

macro_rules! nop881 {
    () => {{
        use core::arch::asm;
        nop440!();
        nop440!();
        asm!("nop");
    }};
}

pub(crate) use nop881;

macro_rules! nop882 {
    () => {{
        use core::arch::asm;
        nop441!();
        nop441!();
    }};
}

pub(crate) use nop882;

macro_rules! nop883 {
    () => {{
        use core::arch::asm;
        nop441!();
        nop441!();
        asm!("nop");
    }};
}

pub(crate) use nop883;

macro_rules! nop884 {
    () => {{
        use core::arch::asm;
        nop442!();
        nop442!();
    }};
}

pub(crate) use nop884;

macro_rules! nop885 {
    () => {{
        use core::arch::asm;
        nop442!();
        nop442!();
        asm!("nop");
    }};
}

pub(crate) use nop885;

macro_rules! nop886 {
    () => {{
        use core::arch::asm;
        nop443!();
        nop443!();
    }};
}

pub(crate) use nop886;

macro_rules! nop887 {
    () => {{
        use core::arch::asm;
        nop443!();
        nop443!();
        asm!("nop");
    }};
}

pub(crate) use nop887;

macro_rules! nop888 {
    () => {{
        use core::arch::asm;
        nop444!();
        nop444!();
    }};
}

pub(crate) use nop888;

macro_rules! nop889 {
    () => {{
        use core::arch::asm;
        nop444!();
        nop444!();
        asm!("nop");
    }};
}

pub(crate) use nop889;

macro_rules! nop890 {
    () => {{
        use core::arch::asm;
        nop445!();
        nop445!();
    }};
}

pub(crate) use nop890;

macro_rules! nop891 {
    () => {{
        use core::arch::asm;
        nop445!();
        nop445!();
        asm!("nop");
    }};
}

pub(crate) use nop891;

macro_rules! nop892 {
    () => {{
        use core::arch::asm;
        nop446!();
        nop446!();
    }};
}

pub(crate) use nop892;

macro_rules! nop893 {
    () => {{
        use core::arch::asm;
        nop446!();
        nop446!();
        asm!("nop");
    }};
}

pub(crate) use nop893;

macro_rules! nop894 {
    () => {{
        use core::arch::asm;
        nop447!();
        nop447!();
    }};
}

pub(crate) use nop894;

macro_rules! nop895 {
    () => {{
        use core::arch::asm;
        nop447!();
        nop447!();
        asm!("nop");
    }};
}

pub(crate) use nop895;

macro_rules! nop896 {
    () => {{
        use core::arch::asm;
        nop448!();
        nop448!();
    }};
}

pub(crate) use nop896;

macro_rules! nop897 {
    () => {{
        use core::arch::asm;
        nop448!();
        nop448!();
        asm!("nop");
    }};
}

pub(crate) use nop897;

macro_rules! nop898 {
    () => {{
        use core::arch::asm;
        nop449!();
        nop449!();
    }};
}

pub(crate) use nop898;

macro_rules! nop899 {
    () => {{
        use core::arch::asm;
        nop449!();
        nop449!();
        asm!("nop");
    }};
}

pub(crate) use nop899;

macro_rules! nop900 {
    () => {{
        use core::arch::asm;
        nop450!();
        nop450!();
    }};
}

pub(crate) use nop900;

macro_rules! nop901 {
    () => {{
        use core::arch::asm;
        nop450!();
        nop450!();
        asm!("nop");
    }};
}

pub(crate) use nop901;

macro_rules! nop902 {
    () => {{
        use core::arch::asm;
        nop451!();
        nop451!();
    }};
}

pub(crate) use nop902;

macro_rules! nop903 {
    () => {{
        use core::arch::asm;
        nop451!();
        nop451!();
        asm!("nop");
    }};
}

pub(crate) use nop903;

macro_rules! nop904 {
    () => {{
        use core::arch::asm;
        nop452!();
        nop452!();
    }};
}

pub(crate) use nop904;

macro_rules! nop905 {
    () => {{
        use core::arch::asm;
        nop452!();
        nop452!();
        asm!("nop");
    }};
}

pub(crate) use nop905;

macro_rules! nop906 {
    () => {{
        use core::arch::asm;
        nop453!();
        nop453!();
    }};
}

pub(crate) use nop906;

macro_rules! nop907 {
    () => {{
        use core::arch::asm;
        nop453!();
        nop453!();
        asm!("nop");
    }};
}

pub(crate) use nop907;

macro_rules! nop908 {
    () => {{
        use core::arch::asm;
        nop454!();
        nop454!();
    }};
}

pub(crate) use nop908;

macro_rules! nop909 {
    () => {{
        use core::arch::asm;
        nop454!();
        nop454!();
        asm!("nop");
    }};
}

pub(crate) use nop909;

macro_rules! nop910 {
    () => {{
        use core::arch::asm;
        nop455!();
        nop455!();
    }};
}

pub(crate) use nop910;

macro_rules! nop911 {
    () => {{
        use core::arch::asm;
        nop455!();
        nop455!();
        asm!("nop");
    }};
}

pub(crate) use nop911;

macro_rules! nop912 {
    () => {{
        use core::arch::asm;
        nop456!();
        nop456!();
    }};
}

pub(crate) use nop912;

macro_rules! nop913 {
    () => {{
        use core::arch::asm;
        nop456!();
        nop456!();
        asm!("nop");
    }};
}

pub(crate) use nop913;

macro_rules! nop914 {
    () => {{
        use core::arch::asm;
        nop457!();
        nop457!();
    }};
}

pub(crate) use nop914;

macro_rules! nop915 {
    () => {{
        use core::arch::asm;
        nop457!();
        nop457!();
        asm!("nop");
    }};
}

pub(crate) use nop915;

macro_rules! nop916 {
    () => {{
        use core::arch::asm;
        nop458!();
        nop458!();
    }};
}

pub(crate) use nop916;

macro_rules! nop917 {
    () => {{
        use core::arch::asm;
        nop458!();
        nop458!();
        asm!("nop");
    }};
}

pub(crate) use nop917;

macro_rules! nop918 {
    () => {{
        use core::arch::asm;
        nop459!();
        nop459!();
    }};
}

pub(crate) use nop918;

macro_rules! nop919 {
    () => {{
        use core::arch::asm;
        nop459!();
        nop459!();
        asm!("nop");
    }};
}

pub(crate) use nop919;

macro_rules! nop920 {
    () => {{
        use core::arch::asm;
        nop460!();
        nop460!();
    }};
}

pub(crate) use nop920;

macro_rules! nop921 {
    () => {{
        use core::arch::asm;
        nop460!();
        nop460!();
        asm!("nop");
    }};
}

pub(crate) use nop921;

macro_rules! nop922 {
    () => {{
        use core::arch::asm;
        nop461!();
        nop461!();
    }};
}

pub(crate) use nop922;

macro_rules! nop923 {
    () => {{
        use core::arch::asm;
        nop461!();
        nop461!();
        asm!("nop");
    }};
}

pub(crate) use nop923;

macro_rules! nop924 {
    () => {{
        use core::arch::asm;
        nop462!();
        nop462!();
    }};
}

pub(crate) use nop924;

macro_rules! nop925 {
    () => {{
        use core::arch::asm;
        nop462!();
        nop462!();
        asm!("nop");
    }};
}

pub(crate) use nop925;

macro_rules! nop926 {
    () => {{
        use core::arch::asm;
        nop463!();
        nop463!();
    }};
}

pub(crate) use nop926;

macro_rules! nop927 {
    () => {{
        use core::arch::asm;
        nop463!();
        nop463!();
        asm!("nop");
    }};
}

pub(crate) use nop927;

macro_rules! nop928 {
    () => {{
        use core::arch::asm;
        nop464!();
        nop464!();
    }};
}

pub(crate) use nop928;

macro_rules! nop929 {
    () => {{
        use core::arch::asm;
        nop464!();
        nop464!();
        asm!("nop");
    }};
}

pub(crate) use nop929;

macro_rules! nop930 {
    () => {{
        use core::arch::asm;
        nop465!();
        nop465!();
    }};
}

pub(crate) use nop930;

macro_rules! nop931 {
    () => {{
        use core::arch::asm;
        nop465!();
        nop465!();
        asm!("nop");
    }};
}

pub(crate) use nop931;

macro_rules! nop932 {
    () => {{
        use core::arch::asm;
        nop466!();
        nop466!();
    }};
}

pub(crate) use nop932;

macro_rules! nop933 {
    () => {{
        use core::arch::asm;
        nop466!();
        nop466!();
        asm!("nop");
    }};
}

pub(crate) use nop933;

macro_rules! nop934 {
    () => {{
        use core::arch::asm;
        nop467!();
        nop467!();
    }};
}

pub(crate) use nop934;

macro_rules! nop935 {
    () => {{
        use core::arch::asm;
        nop467!();
        nop467!();
        asm!("nop");
    }};
}

pub(crate) use nop935;

macro_rules! nop936 {
    () => {{
        use core::arch::asm;
        nop468!();
        nop468!();
    }};
}

pub(crate) use nop936;

macro_rules! nop937 {
    () => {{
        use core::arch::asm;
        nop468!();
        nop468!();
        asm!("nop");
    }};
}

pub(crate) use nop937;

macro_rules! nop938 {
    () => {{
        use core::arch::asm;
        nop469!();
        nop469!();
    }};
}

pub(crate) use nop938;

macro_rules! nop939 {
    () => {{
        use core::arch::asm;
        nop469!();
        nop469!();
        asm!("nop");
    }};
}

pub(crate) use nop939;

macro_rules! nop940 {
    () => {{
        use core::arch::asm;
        nop470!();
        nop470!();
    }};
}

pub(crate) use nop940;

macro_rules! nop941 {
    () => {{
        use core::arch::asm;
        nop470!();
        nop470!();
        asm!("nop");
    }};
}

pub(crate) use nop941;

macro_rules! nop942 {
    () => {{
        use core::arch::asm;
        nop471!();
        nop471!();
    }};
}

pub(crate) use nop942;

macro_rules! nop943 {
    () => {{
        use core::arch::asm;
        nop471!();
        nop471!();
        asm!("nop");
    }};
}

pub(crate) use nop943;

macro_rules! nop944 {
    () => {{
        use core::arch::asm;
        nop472!();
        nop472!();
    }};
}

pub(crate) use nop944;

macro_rules! nop945 {
    () => {{
        use core::arch::asm;
        nop472!();
        nop472!();
        asm!("nop");
    }};
}

pub(crate) use nop945;

macro_rules! nop946 {
    () => {{
        use core::arch::asm;
        nop473!();
        nop473!();
    }};
}

pub(crate) use nop946;

macro_rules! nop947 {
    () => {{
        use core::arch::asm;
        nop473!();
        nop473!();
        asm!("nop");
    }};
}

pub(crate) use nop947;

macro_rules! nop948 {
    () => {{
        use core::arch::asm;
        nop474!();
        nop474!();
    }};
}

pub(crate) use nop948;

macro_rules! nop949 {
    () => {{
        use core::arch::asm;
        nop474!();
        nop474!();
        asm!("nop");
    }};
}

pub(crate) use nop949;

macro_rules! nop950 {
    () => {{
        use core::arch::asm;
        nop475!();
        nop475!();
    }};
}

pub(crate) use nop950;

macro_rules! nop951 {
    () => {{
        use core::arch::asm;
        nop475!();
        nop475!();
        asm!("nop");
    }};
}

pub(crate) use nop951;

macro_rules! nop952 {
    () => {{
        use core::arch::asm;
        nop476!();
        nop476!();
    }};
}

pub(crate) use nop952;

macro_rules! nop953 {
    () => {{
        use core::arch::asm;
        nop476!();
        nop476!();
        asm!("nop");
    }};
}

pub(crate) use nop953;

macro_rules! nop954 {
    () => {{
        use core::arch::asm;
        nop477!();
        nop477!();
    }};
}

pub(crate) use nop954;

macro_rules! nop955 {
    () => {{
        use core::arch::asm;
        nop477!();
        nop477!();
        asm!("nop");
    }};
}

pub(crate) use nop955;

macro_rules! nop956 {
    () => {{
        use core::arch::asm;
        nop478!();
        nop478!();
    }};
}

pub(crate) use nop956;

macro_rules! nop957 {
    () => {{
        use core::arch::asm;
        nop478!();
        nop478!();
        asm!("nop");
    }};
}

pub(crate) use nop957;

macro_rules! nop958 {
    () => {{
        use core::arch::asm;
        nop479!();
        nop479!();
    }};
}

pub(crate) use nop958;

macro_rules! nop959 {
    () => {{
        use core::arch::asm;
        nop479!();
        nop479!();
        asm!("nop");
    }};
}

pub(crate) use nop959;

macro_rules! nop960 {
    () => {{
        use core::arch::asm;
        nop480!();
        nop480!();
    }};
}

pub(crate) use nop960;

macro_rules! nop961 {
    () => {{
        use core::arch::asm;
        nop480!();
        nop480!();
        asm!("nop");
    }};
}

pub(crate) use nop961;

macro_rules! nop962 {
    () => {{
        use core::arch::asm;
        nop481!();
        nop481!();
    }};
}

pub(crate) use nop962;

macro_rules! nop963 {
    () => {{
        use core::arch::asm;
        nop481!();
        nop481!();
        asm!("nop");
    }};
}

pub(crate) use nop963;

macro_rules! nop964 {
    () => {{
        use core::arch::asm;
        nop482!();
        nop482!();
    }};
}

pub(crate) use nop964;

macro_rules! nop965 {
    () => {{
        use core::arch::asm;
        nop482!();
        nop482!();
        asm!("nop");
    }};
}

pub(crate) use nop965;

macro_rules! nop966 {
    () => {{
        use core::arch::asm;
        nop483!();
        nop483!();
    }};
}

pub(crate) use nop966;

macro_rules! nop967 {
    () => {{
        use core::arch::asm;
        nop483!();
        nop483!();
        asm!("nop");
    }};
}

pub(crate) use nop967;

macro_rules! nop968 {
    () => {{
        use core::arch::asm;
        nop484!();
        nop484!();
    }};
}

pub(crate) use nop968;

macro_rules! nop969 {
    () => {{
        use core::arch::asm;
        nop484!();
        nop484!();
        asm!("nop");
    }};
}

pub(crate) use nop969;

macro_rules! nop970 {
    () => {{
        use core::arch::asm;
        nop485!();
        nop485!();
    }};
}

pub(crate) use nop970;

macro_rules! nop971 {
    () => {{
        use core::arch::asm;
        nop485!();
        nop485!();
        asm!("nop");
    }};
}

pub(crate) use nop971;

macro_rules! nop972 {
    () => {{
        use core::arch::asm;
        nop486!();
        nop486!();
    }};
}

pub(crate) use nop972;

macro_rules! nop973 {
    () => {{
        use core::arch::asm;
        nop486!();
        nop486!();
        asm!("nop");
    }};
}

pub(crate) use nop973;

macro_rules! nop974 {
    () => {{
        use core::arch::asm;
        nop487!();
        nop487!();
    }};
}

pub(crate) use nop974;

macro_rules! nop975 {
    () => {{
        use core::arch::asm;
        nop487!();
        nop487!();
        asm!("nop");
    }};
}

pub(crate) use nop975;

macro_rules! nop976 {
    () => {{
        use core::arch::asm;
        nop488!();
        nop488!();
    }};
}

pub(crate) use nop976;

macro_rules! nop977 {
    () => {{
        use core::arch::asm;
        nop488!();
        nop488!();
        asm!("nop");
    }};
}

pub(crate) use nop977;

macro_rules! nop978 {
    () => {{
        use core::arch::asm;
        nop489!();
        nop489!();
    }};
}

pub(crate) use nop978;

macro_rules! nop979 {
    () => {{
        use core::arch::asm;
        nop489!();
        nop489!();
        asm!("nop");
    }};
}

pub(crate) use nop979;

macro_rules! nop980 {
    () => {{
        use core::arch::asm;
        nop490!();
        nop490!();
    }};
}

pub(crate) use nop980;

macro_rules! nop981 {
    () => {{
        use core::arch::asm;
        nop490!();
        nop490!();
        asm!("nop");
    }};
}

pub(crate) use nop981;

macro_rules! nop982 {
    () => {{
        use core::arch::asm;
        nop491!();
        nop491!();
    }};
}

pub(crate) use nop982;

macro_rules! nop983 {
    () => {{
        use core::arch::asm;
        nop491!();
        nop491!();
        asm!("nop");
    }};
}

pub(crate) use nop983;

macro_rules! nop984 {
    () => {{
        use core::arch::asm;
        nop492!();
        nop492!();
    }};
}

pub(crate) use nop984;

macro_rules! nop985 {
    () => {{
        use core::arch::asm;
        nop492!();
        nop492!();
        asm!("nop");
    }};
}

pub(crate) use nop985;

macro_rules! nop986 {
    () => {{
        use core::arch::asm;
        nop493!();
        nop493!();
    }};
}

pub(crate) use nop986;

macro_rules! nop987 {
    () => {{
        use core::arch::asm;
        nop493!();
        nop493!();
        asm!("nop");
    }};
}

pub(crate) use nop987;

macro_rules! nop988 {
    () => {{
        use core::arch::asm;
        nop494!();
        nop494!();
    }};
}

pub(crate) use nop988;

macro_rules! nop989 {
    () => {{
        use core::arch::asm;
        nop494!();
        nop494!();
        asm!("nop");
    }};
}

pub(crate) use nop989;

macro_rules! nop990 {
    () => {{
        use core::arch::asm;
        nop495!();
        nop495!();
    }};
}

pub(crate) use nop990;

macro_rules! nop991 {
    () => {{
        use core::arch::asm;
        nop495!();
        nop495!();
        asm!("nop");
    }};
}

pub(crate) use nop991;

macro_rules! nop992 {
    () => {{
        use core::arch::asm;
        nop496!();
        nop496!();
    }};
}

pub(crate) use nop992;

macro_rules! nop993 {
    () => {{
        use core::arch::asm;
        nop496!();
        nop496!();
        asm!("nop");
    }};
}

pub(crate) use nop993;

macro_rules! nop994 {
    () => {{
        use core::arch::asm;
        nop497!();
        nop497!();
    }};
}

pub(crate) use nop994;

macro_rules! nop995 {
    () => {{
        use core::arch::asm;
        nop497!();
        nop497!();
        asm!("nop");
    }};
}

pub(crate) use nop995;

macro_rules! nop996 {
    () => {{
        use core::arch::asm;
        nop498!();
        nop498!();
    }};
}

pub(crate) use nop996;

macro_rules! nop997 {
    () => {{
        use core::arch::asm;
        nop498!();
        nop498!();
        asm!("nop");
    }};
}

pub(crate) use nop997;

macro_rules! nop998 {
    () => {{
        use core::arch::asm;
        nop499!();
        nop499!();
    }};
}

pub(crate) use nop998;

macro_rules! nop999 {
    () => {{
        use core::arch::asm;
        nop499!();
        nop499!();
        asm!("nop");
    }};
}

pub(crate) use nop999;
