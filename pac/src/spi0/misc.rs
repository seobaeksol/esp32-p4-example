#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "Field `DQ_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type DqOeCtrlR = crate::BitReader;
#[doc = "Field `DQ_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type DqOeCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type CkOeCtrlR = crate::BitReader;
#[doc = "Field `CK_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type CkOeCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type CsOeCtrlR = crate::BitReader;
#[doc = "Field `CS_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type CsOeCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUB_PIN` reader - For SPI0, flash is connected to SUBPINs."]
pub type FsubPinR = crate::BitReader;
#[doc = "Field `SSUB_PIN` reader - For SPI0, sram is connected to SUBPINs."]
pub type SsubPinR = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type CkIdleEdgeR = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type CkIdleEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type CsKeepActiveR = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type CsKeepActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn dq_oe_ctrl(&self) -> DqOeCtrlR {
        DqOeCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn ck_oe_ctrl(&self) -> CkOeCtrlR {
        CkOeCtrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_oe_ctrl(&self) -> CsOeCtrlR {
        CsOeCtrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0, flash is connected to SUBPINs."]
    #[inline(always)]
    pub fn fsub_pin(&self) -> FsubPinR {
        FsubPinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0, sram is connected to SUBPINs."]
    #[inline(always)]
    pub fn ssub_pin(&self) -> SsubPinR {
        SsubPinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CkIdleEdgeR {
        CkIdleEdgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CsKeepActiveR {
        CsKeepActiveR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn dq_oe_ctrl(&mut self) -> DqOeCtrlW<'_, MiscSpec> {
        DqOeCtrlW::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn ck_oe_ctrl(&mut self) -> CkOeCtrlW<'_, MiscSpec> {
        CkOeCtrlW::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_oe_ctrl(&mut self) -> CsOeCtrlW<'_, MiscSpec> {
        CsOeCtrlW::new(self, 6)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CkIdleEdgeW<'_, MiscSpec> {
        CkIdleEdgeW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CsKeepActiveW<'_, MiscSpec> {
        CsKeepActiveW::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0x70"]
impl crate::Resettable for MiscSpec {
    const RESET_VALUE: u32 = 0x70;
}
