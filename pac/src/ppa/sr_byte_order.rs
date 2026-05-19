#[doc = "Register `SR_BYTE_ORDER` reader"]
pub type R = crate::R<SrByteOrderSpec>;
#[doc = "Register `SR_BYTE_ORDER` writer"]
pub type W = crate::W<SrByteOrderSpec>;
#[doc = "Field `SR_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SrRxByteSwapEnR = crate::BitReader;
#[doc = "Field `SR_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SrRxByteSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SrRxRgbSwapEnR = crate::BitReader;
#[doc = "Field `SR_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SrRxRgbSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MACRO_BK_RO_BYPASS` reader - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SrMacroBkRoBypassR = crate::BitReader;
#[doc = "Field `SR_MACRO_BK_RO_BYPASS` writer - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SrMacroBkRoBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_BK_SIZE_SEL` reader - sel srm pix_blk size, 0:32x32, 1:16x16"]
pub type SrBkSizeSelR = crate::BitReader;
#[doc = "Field `SR_BK_SIZE_SEL` writer - sel srm pix_blk size, 0:32x32, 1:16x16"]
pub type SrBkSizeSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn sr_rx_byte_swap_en(&self) -> SrRxByteSwapEnR {
        SrRxByteSwapEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn sr_rx_rgb_swap_en(&self) -> SrRxRgbSwapEnR {
        SrRxRgbSwapEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    pub fn sr_macro_bk_ro_bypass(&self) -> SrMacroBkRoBypassR {
        SrMacroBkRoBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sel srm pix_blk size, 0:32x32, 1:16x16"]
    #[inline(always)]
    pub fn sr_bk_size_sel(&self) -> SrBkSizeSelR {
        SrBkSizeSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn sr_rx_byte_swap_en(&mut self) -> SrRxByteSwapEnW<'_, SrByteOrderSpec> {
        SrRxByteSwapEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn sr_rx_rgb_swap_en(&mut self) -> SrRxRgbSwapEnW<'_, SrByteOrderSpec> {
        SrRxRgbSwapEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    pub fn sr_macro_bk_ro_bypass(&mut self) -> SrMacroBkRoBypassW<'_, SrByteOrderSpec> {
        SrMacroBkRoBypassW::new(self, 2)
    }
    #[doc = "Bit 3 - sel srm pix_blk size, 0:32x32, 1:16x16"]
    #[inline(always)]
    pub fn sr_bk_size_sel(&mut self) -> SrBkSizeSelW<'_, SrByteOrderSpec> {
        SrBkSizeSelW::new(self, 3)
    }
}
#[doc = "Scaling and rotating engine byte order register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_byte_order::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_byte_order::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrByteOrderSpec;
impl crate::RegisterSpec for SrByteOrderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_byte_order::R`](R) reader structure"]
impl crate::Readable for SrByteOrderSpec {}
#[doc = "`write(|w| ..)` method takes [`sr_byte_order::W`](W) writer structure"]
impl crate::Writable for SrByteOrderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_BYTE_ORDER to value 0"]
impl crate::Resettable for SrByteOrderSpec {}
