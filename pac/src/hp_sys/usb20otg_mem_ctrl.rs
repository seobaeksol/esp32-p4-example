#[doc = "Register `USB20OTG_MEM_CTRL` reader"]
pub type R = crate::R<Usb20otgMemCtrlSpec>;
#[doc = "Register `USB20OTG_MEM_CTRL` writer"]
pub type W = crate::W<Usb20otgMemCtrlSpec>;
#[doc = "Field `REG_USB20_MEM_CLK_FORCE_ON` reader - NA"]
pub type RegUsb20MemClkForceOnR = crate::BitReader;
#[doc = "Field `REG_USB20_MEM_CLK_FORCE_ON` writer - NA"]
pub type RegUsb20MemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_usb20_mem_clk_force_on(&self) -> RegUsb20MemClkForceOnR {
        RegUsb20MemClkForceOnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_usb20_mem_clk_force_on(
        &mut self,
    ) -> RegUsb20MemClkForceOnW<'_, Usb20otgMemCtrlSpec> {
        RegUsb20MemClkForceOnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`usb20otg_mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb20otg_mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb20otgMemCtrlSpec;
impl crate::RegisterSpec for Usb20otgMemCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb20otg_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for Usb20otgMemCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb20otg_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for Usb20otgMemCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB20OTG_MEM_CTRL to value 0"]
impl crate::Resettable for Usb20otgMemCtrlSpec {}
