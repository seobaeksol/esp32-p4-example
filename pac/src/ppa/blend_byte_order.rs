#[doc = "Register `BLEND_BYTE_ORDER` reader"]
pub type R = crate::R<BlendByteOrderSpec>;
#[doc = "Register `BLEND_BYTE_ORDER` writer"]
pub type W = crate::W<BlendByteOrderSpec>;
#[doc = "Field `BLEND0_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type Blend0RxByteSwapEnR = crate::BitReader;
#[doc = "Field `BLEND0_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type Blend0RxByteSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND1_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type Blend1RxByteSwapEnR = crate::BitReader;
#[doc = "Field `BLEND1_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type Blend1RxByteSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type Blend0RxRgbSwapEnR = crate::BitReader;
#[doc = "Field `BLEND0_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type Blend0RxRgbSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND1_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type Blend1RxRgbSwapEnR = crate::BitReader;
#[doc = "Field `BLEND1_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type Blend1RxRgbSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn blend0_rx_byte_swap_en(&self) -> Blend0RxByteSwapEnR {
        Blend0RxByteSwapEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn blend1_rx_byte_swap_en(&self) -> Blend1RxByteSwapEnR {
        Blend1RxByteSwapEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn blend0_rx_rgb_swap_en(&self) -> Blend0RxRgbSwapEnR {
        Blend0RxRgbSwapEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn blend1_rx_rgb_swap_en(&self) -> Blend1RxRgbSwapEnR {
        Blend1RxRgbSwapEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn blend0_rx_byte_swap_en(&mut self) -> Blend0RxByteSwapEnW<'_, BlendByteOrderSpec> {
        Blend0RxByteSwapEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn blend1_rx_byte_swap_en(&mut self) -> Blend1RxByteSwapEnW<'_, BlendByteOrderSpec> {
        Blend1RxByteSwapEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn blend0_rx_rgb_swap_en(&mut self) -> Blend0RxRgbSwapEnW<'_, BlendByteOrderSpec> {
        Blend0RxRgbSwapEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn blend1_rx_rgb_swap_en(&mut self) -> Blend1RxRgbSwapEnW<'_, BlendByteOrderSpec> {
        Blend1RxRgbSwapEnW::new(self, 3)
    }
}
#[doc = "Blending engine byte order register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_byte_order::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_byte_order::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendByteOrderSpec;
impl crate::RegisterSpec for BlendByteOrderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_byte_order::R`](R) reader structure"]
impl crate::Readable for BlendByteOrderSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_byte_order::W`](W) writer structure"]
impl crate::Writable for BlendByteOrderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_BYTE_ORDER to value 0"]
impl crate::Resettable for BlendByteOrderSpec {}
