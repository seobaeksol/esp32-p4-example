#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type ClkModeR = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type ClkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_HOLD_DLY_RES` reader - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
pub type CsHoldDlyResR = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY_RES` writer - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
pub type CsHoldDlyResW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CS_HOLD_DLY_PER` reader - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[8:0\\] * 128) SPI_CLK cycles."]
pub type CsHoldDlyPerR = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY_PER` writer - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[8:0\\] * 128) SPI_CLK cycles."]
pub type CsHoldDlyPerW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CS_HOLD_DLY_PER_EN` reader - 1: use SPI_MEM_CS_HOLD_DLY_PER for per, use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm . 0: use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm/per ."]
pub type CsHoldDlyPerEnR = crate::BitReader;
#[doc = "Field `CS_HOLD_DLY_PER_EN` writer - 1: use SPI_MEM_CS_HOLD_DLY_PER for per, use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm . 0: use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm/per ."]
pub type CsHoldDlyPerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> ClkModeR {
        ClkModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CsHoldDlyResR {
        CsHoldDlyResR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:20 - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[8:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_per(&self) -> CsHoldDlyPerR {
        CsHoldDlyPerR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 23 - 1: use SPI_MEM_CS_HOLD_DLY_PER for per, use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm . 0: use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm/per ."]
    #[inline(always)]
    pub fn cs_hold_dly_per_en(&self) -> CsHoldDlyPerEnR {
        CsHoldDlyPerEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> ClkModeW<'_, Ctrl1Spec> {
        ClkModeW::new(self, 0)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CsHoldDlyResW<'_, Ctrl1Spec> {
        CsHoldDlyResW::new(self, 2)
    }
    #[doc = "Bits 12:20 - After PER command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DLY_PER\\[8:0\\] * 128) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_per(&mut self) -> CsHoldDlyPerW<'_, Ctrl1Spec> {
        CsHoldDlyPerW::new(self, 12)
    }
    #[doc = "Bit 23 - 1: use SPI_MEM_CS_HOLD_DLY_PER for per, use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm . 0: use SPI_MEM_CS_HOLD_DELAY_RES for pes/dp/hpm/per ."]
    #[inline(always)]
    pub fn cs_hold_dly_per_en(&mut self) -> CsHoldDlyPerEnW<'_, Ctrl1Spec> {
        CsHoldDlyPerEnW::new(self, 23)
    }
}
#[doc = "SPI1 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0x001f_fffc"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0x001f_fffc;
}
