#[doc = "Register `CAM_CNTL` reader"]
pub type R = crate::R<CamCntlSpec>;
#[doc = "Register `CAM_CNTL` writer"]
pub type W = crate::W<CamCntlSpec>;
#[doc = "Field `CAM_EN` reader - write 1 to start recive camera data, write 0 to disable"]
pub type CamEnR = crate::BitReader;
#[doc = "Field `CAM_EN` writer - write 1 to start recive camera data, write 0 to disable"]
pub type CamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_UPDATE` reader - write 1 to update ISP_CAM_CONF"]
pub type CamUpdateR = crate::BitReader;
#[doc = "Field `CAM_UPDATE` writer - write 1 to update ISP_CAM_CONF"]
pub type CamUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_RESET` reader - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CamResetR = crate::BitReader;
#[doc = "Field `CAM_RESET` writer - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CamResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CLK_INV` reader - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CamClkInvR = crate::BitReader;
#[doc = "Field `CAM_CLK_INV` writer - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CamClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    pub fn cam_en(&self) -> CamEnR {
        CamEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    pub fn cam_update(&self) -> CamUpdateR {
        CamUpdateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    pub fn cam_reset(&self) -> CamResetR {
        CamResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> CamClkInvR {
        CamClkInvR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    pub fn cam_en(&mut self) -> CamEnW<'_, CamCntlSpec> {
        CamEnW::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    pub fn cam_update(&mut self) -> CamUpdateW<'_, CamCntlSpec> {
        CamUpdateW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    pub fn cam_reset(&mut self) -> CamResetW<'_, CamCntlSpec> {
        CamResetW::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    pub fn cam_clk_inv(&mut self) -> CamClkInvW<'_, CamCntlSpec> {
        CamClkInvW::new(self, 3)
    }
}
#[doc = "isp cam source control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CamCntlSpec;
impl crate::RegisterSpec for CamCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_cntl::R`](R) reader structure"]
impl crate::Readable for CamCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`cam_cntl::W`](W) writer structure"]
impl crate::Writable for CamCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAM_CNTL to value 0x04"]
impl crate::Resettable for CamCntlSpec {
    const RESET_VALUE: u32 = 0x04;
}
