#[doc = "Register `PMUP_CHANNEL_CFG` reader"]
pub type R = crate::R<PmupChannelCfgSpec>;
#[doc = "Register `PMUP_CHANNEL_CFG` writer"]
pub type W = crate::W<PmupChannelCfgSpec>;
#[doc = "Field `PUMP_CHANNEL_CODE4` reader - configure cmd4 code"]
pub type PumpChannelCode4R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE4` writer - configure cmd4 code"]
pub type PumpChannelCode4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE3` reader - configure cmd3 code"]
pub type PumpChannelCode3R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE3` writer - configure cmd3 code"]
pub type PumpChannelCode3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE2` reader - configure cmd2 code"]
pub type PumpChannelCode2R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE2` writer - configure cmd2 code"]
pub type PumpChannelCode2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE1` reader - configure cmd1 code"]
pub type PumpChannelCode1R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE1` writer - configure cmd1 code"]
pub type PumpChannelCode1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE0` reader - configure cmd0 code"]
pub type PumpChannelCode0R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE0` writer - configure cmd0 code"]
pub type PumpChannelCode0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 7:11 - configure cmd4 code"]
    #[inline(always)]
    pub fn pump_channel_code4(&self) -> PumpChannelCode4R {
        PumpChannelCode4R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - configure cmd3 code"]
    #[inline(always)]
    pub fn pump_channel_code3(&self) -> PumpChannelCode3R {
        PumpChannelCode3R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - configure cmd2 code"]
    #[inline(always)]
    pub fn pump_channel_code2(&self) -> PumpChannelCode2R {
        PumpChannelCode2R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - configure cmd1 code"]
    #[inline(always)]
    pub fn pump_channel_code1(&self) -> PumpChannelCode1R {
        PumpChannelCode1R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - configure cmd0 code"]
    #[inline(always)]
    pub fn pump_channel_code0(&self) -> PumpChannelCode0R {
        PumpChannelCode0R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:11 - configure cmd4 code"]
    #[inline(always)]
    pub fn pump_channel_code4(&mut self) -> PumpChannelCode4W<'_, PmupChannelCfgSpec> {
        PumpChannelCode4W::new(self, 7)
    }
    #[doc = "Bits 12:16 - configure cmd3 code"]
    #[inline(always)]
    pub fn pump_channel_code3(&mut self) -> PumpChannelCode3W<'_, PmupChannelCfgSpec> {
        PumpChannelCode3W::new(self, 12)
    }
    #[doc = "Bits 17:21 - configure cmd2 code"]
    #[inline(always)]
    pub fn pump_channel_code2(&mut self) -> PumpChannelCode2W<'_, PmupChannelCfgSpec> {
        PumpChannelCode2W::new(self, 17)
    }
    #[doc = "Bits 22:26 - configure cmd1 code"]
    #[inline(always)]
    pub fn pump_channel_code1(&mut self) -> PumpChannelCode1W<'_, PmupChannelCfgSpec> {
        PumpChannelCode1W::new(self, 22)
    }
    #[doc = "Bits 27:31 - configure cmd0 code"]
    #[inline(always)]
    pub fn pump_channel_code0(&mut self) -> PumpChannelCode0W<'_, PmupChannelCfgSpec> {
        PumpChannelCode0W::new(self, 27)
    }
}
#[doc = "configure the code of valid pump channel code\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_channel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_channel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmupChannelCfgSpec;
impl crate::RegisterSpec for PmupChannelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_channel_cfg::R`](R) reader structure"]
impl crate::Readable for PmupChannelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmup_channel_cfg::W`](W) writer structure"]
impl crate::Writable for PmupChannelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUP_CHANNEL_CFG to value 0"]
impl crate::Resettable for PmupChannelCfgSpec {}
