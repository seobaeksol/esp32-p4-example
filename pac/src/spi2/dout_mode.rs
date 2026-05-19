#[doc = "Register `DOUT_MODE` reader"]
pub type R = crate::R<DoutModeSpec>;
#[doc = "Register `DOUT_MODE` writer"]
pub type W = crate::W<DoutModeSpec>;
#[doc = "Field `DOUT0_MODE` reader - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout0ModeR = crate::BitReader;
#[doc = "Field `DOUT0_MODE` writer - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout0ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT1_MODE` reader - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout1ModeR = crate::BitReader;
#[doc = "Field `DOUT1_MODE` writer - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout1ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT2_MODE` reader - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout2ModeR = crate::BitReader;
#[doc = "Field `DOUT2_MODE` writer - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT3_MODE` reader - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout3ModeR = crate::BitReader;
#[doc = "Field `DOUT3_MODE` writer - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout3ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT4_MODE` reader - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout4ModeR = crate::BitReader;
#[doc = "Field `DOUT4_MODE` writer - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout4ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT5_MODE` reader - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout5ModeR = crate::BitReader;
#[doc = "Field `DOUT5_MODE` writer - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout5ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT6_MODE` reader - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout6ModeR = crate::BitReader;
#[doc = "Field `DOUT6_MODE` writer - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout6ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT7_MODE` reader - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout7ModeR = crate::BitReader;
#[doc = "Field `DOUT7_MODE` writer - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type Dout7ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_DQS_MODE` reader - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type DDqsModeR = crate::BitReader;
#[doc = "Field `D_DQS_MODE` writer - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
pub type DDqsModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> Dout0ModeR {
        Dout0ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> Dout1ModeR {
        Dout1ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> Dout2ModeR {
        Dout2ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> Dout3ModeR {
        Dout3ModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> Dout4ModeR {
        Dout4ModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> Dout5ModeR {
        Dout5ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> Dout6ModeR {
        Dout6ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> Dout7ModeR {
        Dout7ModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&self) -> DDqsModeR {
        DDqsModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> Dout0ModeW<'_, DoutModeSpec> {
        Dout0ModeW::new(self, 0)
    }
    #[doc = "Bit 1 - The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> Dout1ModeW<'_, DoutModeSpec> {
        Dout1ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> Dout2ModeW<'_, DoutModeSpec> {
        Dout2ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> Dout3ModeW<'_, DoutModeSpec> {
        Dout3ModeW::new(self, 3)
    }
    #[doc = "Bit 4 - The output signal 4 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_mode(&mut self) -> Dout4ModeW<'_, DoutModeSpec> {
        Dout4ModeW::new(self, 4)
    }
    #[doc = "Bit 5 - The output signal 5 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_mode(&mut self) -> Dout5ModeW<'_, DoutModeSpec> {
        Dout5ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - The output signal 6 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_mode(&mut self) -> Dout6ModeW<'_, DoutModeSpec> {
        Dout6ModeW::new(self, 6)
    }
    #[doc = "Bit 7 - The output signal 7 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_mode(&mut self) -> Dout7ModeW<'_, DoutModeSpec> {
        Dout7ModeW::new(self, 7)
    }
    #[doc = "Bit 8 - The output signal SPI_DQS is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_mode(&mut self) -> DDqsModeW<'_, DoutModeSpec> {
        DDqsModeW::new(self, 8)
    }
}
#[doc = "SPI output delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutModeSpec;
impl crate::RegisterSpec for DoutModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_mode::R`](R) reader structure"]
impl crate::Readable for DoutModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dout_mode::W`](W) writer structure"]
impl crate::Writable for DoutModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DoutModeSpec {}
