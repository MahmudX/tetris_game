use sdl2::pixels::Color;
#[derive(Clone, Copy)]
pub enum TextureColor {
    Slate50, Slate100, Slate200, Slate300, Slate400, Slate500,
    Slate600, Slate700, Slate800, Slate900, Slate950,
    Gray50, Gray100, Gray200, Gray300, Gray400, Gray500,
    Gray600, Gray700, Gray800, Gray900, Gray950,
    Zinc50, Zinc100, Zinc200, Zinc300, Zinc400, Zinc500,
    Zinc600, Zinc700, Zinc800, Zinc900, Zinc950,
    Neutral50, Neutral100, Neutral200, Neutral300, Neutral400, Neutral500,
    Neutral600, Neutral700, Neutral800, Neutral900, Neutral950,
    Stone50, Stone100, Stone200, Stone300, Stone400, Stone500,
    Stone600, Stone700, Stone800, Stone900, Stone950,
    Red50, Red100, Red200, Red300, Red400, Red500,
    Red600, Red700, Red800, Red900, Red950,
    Orange50, Orange100, Orange200, Orange300, Orange400, Orange500,
    Orange600, Orange700, Orange800, Orange900, Orange950,
    Amber50, Amber100, Amber200, Amber300, Amber400, Amber500,
    Amber600, Amber700, Amber800, Amber900, Amber950,
    Yellow50, Yellow100, Yellow200, Yellow300, Yellow400, Yellow500,
    Yellow600, Yellow700, Yellow800, Yellow900, Yellow950,
    Lime50, Lime100, Lime200, Lime300, Lime400, Lime500,
    Lime600, Lime700, Lime800, Lime900, Lime950,
    Green50, Green100, Green200, Green300, Green400, Green500,
    Green600, Green700, Green800, Green900, Green950,
    Emerald50, Emerald100, Emerald200, Emerald300, Emerald400, Emerald500,
    Emerald600, Emerald700, Emerald800, Emerald900, Emerald950,
    Teal50, Teal100, Teal200, Teal300, Teal400, Teal500,
    Teal600, Teal700, Teal800, Teal900, Teal950,
    Cyan50, Cyan100, Cyan200, Cyan300, Cyan400, Cyan500,
    Cyan600, Cyan700, Cyan800, Cyan900, Cyan950,
    Sky50, Sky100, Sky200, Sky300, Sky400, Sky500,
    Sky600, Sky700, Sky800, Sky900, Sky950,
    Blue50, Blue100, Blue200, Blue300, Blue400, Blue500,
    Blue600, Blue700, Blue800, Blue900, Blue950,
    Indigo50, Indigo100, Indigo200, Indigo300, Indigo400, Indigo500,
    Indigo600, Indigo700, Indigo800, Indigo900, Indigo950,
    Violet50, Violet100, Violet200, Violet300, Violet400, Violet500,
    Violet600, Violet700, Violet800, Violet900, Violet950,
    Purple50, Purple100, Purple200, Purple300, Purple400, Purple500,
    Purple600, Purple700, Purple800, Purple900, Purple950,
    Fuchsia50, Fuchsia100, Fuchsia200, Fuchsia300, Fuchsia400, Fuchsia500,
    Fuchsia600, Fuchsia700, Fuchsia800, Fuchsia900, Fuchsia950,
    Pink50, Pink100, Pink200, Pink300, Pink400, Pink500,
    Pink600, Pink700, Pink800, Pink900, Pink950,
    Rose50, Rose100, Rose200, Rose300, Rose400, Rose500,
    Rose600, Rose700, Rose800, Rose900, Rose950
}

pub fn get_rgb_color(color: TextureColor) -> Color {
    match color {
        TextureColor::Slate50 => {Color::RGB(248, 250, 252)},
        TextureColor::Slate100 => {Color::RGB(241, 245, 249)},
        TextureColor::Slate200 => {Color::RGB(226, 232, 240)},
        TextureColor::Slate300 => {Color::RGB(203, 213, 225)},
        TextureColor::Slate400 => {Color::RGB(148, 163, 184)},
        TextureColor::Slate500 => {Color::RGB(100, 116, 139)},
        TextureColor::Slate600 => {Color::RGB(71, 85, 105)},
        TextureColor::Slate700 => {Color::RGB(51, 65, 85)},
        TextureColor::Slate800 => {Color::RGB(30, 41, 59)},
        TextureColor::Slate900 => {Color::RGB(15, 23, 42)},
        TextureColor::Slate950 => {Color::RGB(2, 6, 23)},
        TextureColor::Gray50 => {Color::RGB(249, 250, 251)},
        TextureColor::Gray100 => {Color::RGB(243, 244, 246)},
        TextureColor::Gray200 => {Color::RGB(229, 231, 235)},
        TextureColor::Gray300 => {Color::RGB(209, 213, 219)},
        TextureColor::Gray400 => {Color::RGB(156, 163, 175)},
        TextureColor::Gray500 => {Color::RGB(107, 114, 128)},
        TextureColor::Gray600 => {Color::RGB(75, 85, 99)},
        TextureColor::Gray700 => {Color::RGB(55, 65, 81)},
        TextureColor::Gray800 => {Color::RGB(31, 41, 55)},
        TextureColor::Gray900 => {Color::RGB(17, 24, 39)},
        TextureColor::Gray950 => {Color::RGB(3, 7, 18)},
        TextureColor::Zinc50 => {Color::RGB(250, 250, 250)},
        TextureColor::Zinc100 => {Color::RGB(244, 244, 245)},
        TextureColor::Zinc200 => {Color::RGB(228, 228, 231)},
        TextureColor::Zinc300 => {Color::RGB(212, 212, 216)},
        TextureColor::Zinc400 => {Color::RGB(161, 161, 170)},
        TextureColor::Zinc500 => {Color::RGB(113, 113, 122)},
        TextureColor::Zinc600 => {Color::RGB(82, 82, 91)},
        TextureColor::Zinc700 => {Color::RGB(63, 63, 70)},
        TextureColor::Zinc800 => {Color::RGB(39, 39, 42)},
        TextureColor::Zinc900 => {Color::RGB(24, 24, 27)},
        TextureColor::Zinc950 => {Color::RGB(9, 9, 11)},
        TextureColor::Neutral50 => {Color::RGB(250, 250, 250)},
        TextureColor::Neutral100 => {Color::RGB(245, 245, 245)},
        TextureColor::Neutral200 => {Color::RGB(229, 229, 229)},
        TextureColor::Neutral300 => {Color::RGB(212, 212, 212)},
        TextureColor::Neutral400 => {Color::RGB(163, 163, 163)},
        TextureColor::Neutral500 => {Color::RGB(115, 115, 115)},
        TextureColor::Neutral600 => {Color::RGB(82, 82, 82)},
        TextureColor::Neutral700 => {Color::RGB(64, 64, 64)},
        TextureColor::Neutral800 => {Color::RGB(38, 38, 38)},
        TextureColor::Neutral900 => {Color::RGB(23, 23, 23)},
        TextureColor::Neutral950 => {Color::RGB(10, 10, 10)},
        TextureColor::Stone50 => {Color::RGB(250, 250, 249)},
        TextureColor::Stone100 => {Color::RGB(245, 245, 244)},
        TextureColor::Stone200 => {Color::RGB(231, 229, 228)},
        TextureColor::Stone300 => {Color::RGB(214, 211, 209)},
        TextureColor::Stone400 => {Color::RGB(168, 162, 158)},
        TextureColor::Stone500 => {Color::RGB(120, 113, 108)},
        TextureColor::Stone600 => {Color::RGB(87, 83, 78)},
        TextureColor::Stone700 => {Color::RGB(68, 64, 60)},
        TextureColor::Stone800 => {Color::RGB(41, 37, 36)},
        TextureColor::Stone900 => {Color::RGB(28, 25, 23)},
        TextureColor::Stone950 => {Color::RGB(12, 10, 9)},
        TextureColor::Red50 => {Color::RGB(254, 242, 242)},
        TextureColor::Red100 => {Color::RGB(254, 226, 226)},
        TextureColor::Red200 => {Color::RGB(254, 202, 202)},
        TextureColor::Red300 => {Color::RGB(252, 165, 165)},
        TextureColor::Red400 => {Color::RGB(248, 113, 113)},
        TextureColor::Red500 => {Color::RGB(239, 68, 68)},
        TextureColor::Red600 => {Color::RGB(220, 38, 38)},
        TextureColor::Red700 => {Color::RGB(185, 28, 28)},
        TextureColor::Red800 => {Color::RGB(153, 27, 27)},
        TextureColor::Red900 => {Color::RGB(127, 29, 29)},
        TextureColor::Red950 => {Color::RGB(69, 10, 10)},
        TextureColor::Orange50 => {Color::RGB(255, 247, 237)},
        TextureColor::Orange100 => {Color::RGB(255, 237, 213)},
        TextureColor::Orange200 => {Color::RGB(254, 215, 170)},
        TextureColor::Orange300 => {Color::RGB(253, 186, 116)},
        TextureColor::Orange400 => {Color::RGB(251, 146, 60)},
        TextureColor::Orange500 => {Color::RGB(249, 115, 22)},
        TextureColor::Orange600 => {Color::RGB(234, 88, 12)},
        TextureColor::Orange700 => {Color::RGB(194, 65, 12)},
        TextureColor::Orange800 => {Color::RGB(154, 52, 18)},
        TextureColor::Orange900 => {Color::RGB(124, 45, 18)},
        TextureColor::Orange950 => {Color::RGB(67, 20, 7)},
        TextureColor::Amber50 => {Color::RGB(255, 251, 235)},
        TextureColor::Amber100 => {Color::RGB(254, 243, 199)},
        TextureColor::Amber200 => {Color::RGB(253, 230, 138)},
        TextureColor::Amber300 => {Color::RGB(252, 211, 77)},
        TextureColor::Amber400 => {Color::RGB(251, 191, 36)},
        TextureColor::Amber500 => {Color::RGB(245, 158, 11)},
        TextureColor::Amber600 => {Color::RGB(217, 119, 6)},
        TextureColor::Amber700 => {Color::RGB(180, 83, 9)},
        TextureColor::Amber800 => {Color::RGB(146, 64, 14)},
        TextureColor::Amber900 => {Color::RGB(120, 53, 15)},
        TextureColor::Amber950 => {Color::RGB(69, 26, 3)},
        TextureColor::Yellow50 => {Color::RGB(254, 252, 232)},
        TextureColor::Yellow100 => {Color::RGB(254, 249, 195)},
        TextureColor::Yellow200 => {Color::RGB(254, 240, 138)},
        TextureColor::Yellow300 => {Color::RGB(253, 224, 71)},
        TextureColor::Yellow400 => {Color::RGB(250, 204, 21)},
        TextureColor::Yellow500 => {Color::RGB(234, 179, 8)},
        TextureColor::Yellow600 => {Color::RGB(202, 138, 4)},
        TextureColor::Yellow700 => {Color::RGB(161, 98, 7)},
        TextureColor::Yellow800 => {Color::RGB(133, 77, 14)},
        TextureColor::Yellow900 => {Color::RGB(113, 63, 18)},
        TextureColor::Yellow950 => {Color::RGB(66, 32, 6)},
        TextureColor::Lime50 => {Color::RGB(247, 254, 231)},
        TextureColor::Lime100 => {Color::RGB(236, 252, 203)},
        TextureColor::Lime200 => {Color::RGB(217, 249, 157)},
        TextureColor::Lime300 => {Color::RGB(190, 242, 100)},
        TextureColor::Lime400 => {Color::RGB(163, 230, 53)},
        TextureColor::Lime500 => {Color::RGB(132, 204, 22)},
        TextureColor::Lime600 => {Color::RGB(101, 163, 13)},
        TextureColor::Lime700 => {Color::RGB(77, 124, 15)},
        TextureColor::Lime800 => {Color::RGB(63, 98, 18)},
        TextureColor::Lime900 => {Color::RGB(54, 83, 20)},
        TextureColor::Lime950 => {Color::RGB(26, 46, 5)},
        TextureColor::Green50 => {Color::RGB(240, 253, 244)},
        TextureColor::Green100 => {Color::RGB(220, 252, 231)},
        TextureColor::Green200 => {Color::RGB(187, 247, 208)},
        TextureColor::Green300 => {Color::RGB(134, 239, 172)},
        TextureColor::Green400 => {Color::RGB(74, 222, 128)},
        TextureColor::Green500 => {Color::RGB(34, 197, 94)},
        TextureColor::Green600 => {Color::RGB(22, 163, 74)},
        TextureColor::Green700 => {Color::RGB(21, 128, 61)},
        TextureColor::Green800 => {Color::RGB(22, 101, 52)},
        TextureColor::Green900 => {Color::RGB(20, 83, 45)},
        TextureColor::Green950 => {Color::RGB(5, 46, 22)},
        TextureColor::Emerald50 => {Color::RGB(236, 253, 245)},
        TextureColor::Emerald100 => {Color::RGB(209, 250, 229)},
        TextureColor::Emerald200 => {Color::RGB(167, 243, 208)},
        TextureColor::Emerald300 => {Color::RGB(110, 231, 183)},
        TextureColor::Emerald400 => {Color::RGB(52, 211, 153)},
        TextureColor::Emerald500 => {Color::RGB(16, 185, 129)},
        TextureColor::Emerald600 => {Color::RGB(5, 150, 105)},
        TextureColor::Emerald700 => {Color::RGB(4, 120, 87)},
        TextureColor::Emerald800 => {Color::RGB(6, 95, 70)},
        TextureColor::Emerald900 => {Color::RGB(6, 78, 59)},
        TextureColor::Emerald950 => {Color::RGB(2, 44, 34)},
        TextureColor::Teal50 => {Color::RGB(240, 253, 250)},
        TextureColor::Teal100 => {Color::RGB(204, 251, 241)},
        TextureColor::Teal200 => {Color::RGB(153, 246, 228)},
        TextureColor::Teal300 => {Color::RGB(94, 234, 212)},
        TextureColor::Teal400 => {Color::RGB(45, 212, 191)},
        TextureColor::Teal500 => {Color::RGB(20, 184, 166)},
        TextureColor::Teal600 => {Color::RGB(13, 148, 136)},
        TextureColor::Teal700 => {Color::RGB(15, 118, 110)},
        TextureColor::Teal800 => {Color::RGB(17, 94, 89)},
        TextureColor::Teal900 => {Color::RGB(19, 78, 74)},
        TextureColor::Teal950 => {Color::RGB(4, 47, 46)},
        TextureColor::Cyan50 => {Color::RGB(236, 254, 255)},
        TextureColor::Cyan100 => {Color::RGB(207, 250, 254)},
        TextureColor::Cyan200 => {Color::RGB(165, 243, 252)},
        TextureColor::Cyan300 => {Color::RGB(103, 232, 249)},
        TextureColor::Cyan400 => {Color::RGB(34, 211, 238)},
        TextureColor::Cyan500 => {Color::RGB(6, 182, 212)},
        TextureColor::Cyan600 => {Color::RGB(8, 145, 178)},
        TextureColor::Cyan700 => {Color::RGB(14, 116, 144)},
        TextureColor::Cyan800 => {Color::RGB(21, 94, 117)},
        TextureColor::Cyan900 => {Color::RGB(22, 78, 99)},
        TextureColor::Cyan950 => {Color::RGB(8, 51, 68)},
        TextureColor::Sky50 => {Color::RGB(240, 249, 255)},
        TextureColor::Sky100 => {Color::RGB(224, 242, 254)},
        TextureColor::Sky200 => {Color::RGB(186, 230, 253)},
        TextureColor::Sky300 => {Color::RGB(125, 211, 252)},
        TextureColor::Sky400 => {Color::RGB(56, 189, 248)},
        TextureColor::Sky500 => {Color::RGB(14, 165, 233)},
        TextureColor::Sky600 => {Color::RGB(2, 132, 199)},
        TextureColor::Sky700 => {Color::RGB(3, 105, 161)},
        TextureColor::Sky800 => {Color::RGB(7, 89, 133)},
        TextureColor::Sky900 => {Color::RGB(12, 74, 110)},
        TextureColor::Sky950 => {Color::RGB(8, 47, 73)},
        TextureColor::Blue50 => {Color::RGB(239, 246, 255)},
        TextureColor::Blue100 => {Color::RGB(219, 234, 254)},
        TextureColor::Blue200 => {Color::RGB(191, 219, 254)},
        TextureColor::Blue300 => {Color::RGB(147, 197, 253)},
        TextureColor::Blue400 => {Color::RGB(96, 165, 250)},
        TextureColor::Blue500 => {Color::RGB(59, 130, 246)},
        TextureColor::Blue600 => {Color::RGB(37, 99, 235)},
        TextureColor::Blue700 => {Color::RGB(29, 78, 216)},
        TextureColor::Blue800 => {Color::RGB(30, 64, 175)},
        TextureColor::Blue900 => {Color::RGB(30, 58, 138)},
        TextureColor::Blue950 => {Color::RGB(23, 37, 84)},
        TextureColor::Indigo50 => {Color::RGB(238, 242, 255)},
        TextureColor::Indigo100 => {Color::RGB(224, 231, 255)},
        TextureColor::Indigo200 => {Color::RGB(199, 210, 254)},
        TextureColor::Indigo300 => {Color::RGB(165, 180, 252)},
        TextureColor::Indigo400 => {Color::RGB(129, 140, 248)},
        TextureColor::Indigo500 => {Color::RGB(99, 102, 241)},
        TextureColor::Indigo600 => {Color::RGB(79, 70, 229)},
        TextureColor::Indigo700 => {Color::RGB(67, 56, 202)},
        TextureColor::Indigo800 => {Color::RGB(55, 48, 163)},
        TextureColor::Indigo900 => {Color::RGB(49, 46, 129)},
        TextureColor::Indigo950 => {Color::RGB(30, 27, 75)},
        TextureColor::Violet50 => {Color::RGB(245, 243, 255)},
        TextureColor::Violet100 => {Color::RGB(237, 233, 254)},
        TextureColor::Violet200 => {Color::RGB(221, 214, 254)},
        TextureColor::Violet300 => {Color::RGB(196, 181, 253)},
        TextureColor::Violet400 => {Color::RGB(167, 139, 250)},
        TextureColor::Violet500 => {Color::RGB(139, 92, 246)},
        TextureColor::Violet600 => {Color::RGB(124, 58, 237)},
        TextureColor::Violet700 => {Color::RGB(109, 40, 217)},
        TextureColor::Violet800 => {Color::RGB(91, 33, 182)},
        TextureColor::Violet900 => {Color::RGB(76, 29, 149)},
        TextureColor::Violet950 => {Color::RGB(46, 16, 101)},
        TextureColor::Purple50 => {Color::RGB(250, 245, 255)},
        TextureColor::Purple100 => {Color::RGB(243, 232, 255)},
        TextureColor::Purple200 => {Color::RGB(233, 213, 255)},
        TextureColor::Purple300 => {Color::RGB(216, 180, 254)},
        TextureColor::Purple400 => {Color::RGB(192, 132, 252)},
        TextureColor::Purple500 => {Color::RGB(168, 85, 247)},
        TextureColor::Purple600 => {Color::RGB(147, 51, 234)},
        TextureColor::Purple700 => {Color::RGB(126, 34, 206)},
        TextureColor::Purple800 => {Color::RGB(107, 33, 168)},
        TextureColor::Purple900 => {Color::RGB(88, 28, 135)},
        TextureColor::Purple950 => {Color::RGB(59, 7, 100)},
        TextureColor::Fuchsia50 => {Color::RGB(253, 244, 255)},
        TextureColor::Fuchsia100 => {Color::RGB(250, 232, 255)},
        TextureColor::Fuchsia200 => {Color::RGB(245, 208, 254)},
        TextureColor::Fuchsia300 => {Color::RGB(240, 171, 252)},
        TextureColor::Fuchsia400 => {Color::RGB(232, 121, 249)},
        TextureColor::Fuchsia500 => {Color::RGB(217, 70, 239)},
        TextureColor::Fuchsia600 => {Color::RGB(192, 38, 211)},
        TextureColor::Fuchsia700 => {Color::RGB(162, 28, 175)},
        TextureColor::Fuchsia800 => {Color::RGB(134, 25, 143)},
        TextureColor::Fuchsia900 => {Color::RGB(112, 26, 117)},
        TextureColor::Fuchsia950 => {Color::RGB(74, 4, 78)},
        TextureColor::Pink50 => {Color::RGB(253, 242, 248)},
        TextureColor::Pink100 => {Color::RGB(252, 231, 243)},
        TextureColor::Pink200 => {Color::RGB(251, 207, 232)},
        TextureColor::Pink300 => {Color::RGB(249, 168, 212)},
        TextureColor::Pink400 => {Color::RGB(244, 114, 182)},
        TextureColor::Pink500 => {Color::RGB(236, 72, 153)},
        TextureColor::Pink600 => {Color::RGB(219, 39, 119)},
        TextureColor::Pink700 => {Color::RGB(190, 24, 93)},
        TextureColor::Pink800 => {Color::RGB(157, 23, 77)},
        TextureColor::Pink900 => {Color::RGB(131, 24, 67)},
        TextureColor::Pink950 => {Color::RGB(80, 7, 36)},
        TextureColor::Rose50 => {Color::RGB(255, 241, 242)},
        TextureColor::Rose100 => {Color::RGB(255, 228, 230)},
        TextureColor::Rose200 => {Color::RGB(254, 205, 211)},
        TextureColor::Rose300 => {Color::RGB(253, 164, 175)},
        TextureColor::Rose400 => {Color::RGB(251, 113, 133)},
        TextureColor::Rose500 => {Color::RGB(244, 63, 94)},
        TextureColor::Rose600 => {Color::RGB(225, 29, 72)},
        TextureColor::Rose700 => {Color::RGB(190, 18, 60)},
        TextureColor::Rose800 => {Color::RGB(159, 18, 57)},
        TextureColor::Rose900 => {Color::RGB(136, 19, 55)},
        TextureColor::Rose950 => {Color::RGB(76, 5, 25)},
    }
}