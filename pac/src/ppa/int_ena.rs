#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `SR_EOF` reader - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
pub type SrEofR = crate::BitReader;
#[doc = "Field `SR_EOF` writer - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
pub type SrEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_EOF` reader - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BlendEofR = crate::BitReader;
#[doc = "Field `BLEND_EOF` writer - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BlendEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_PARAM_CFG_ERR` reader - The interrupt enable bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
pub type SrParamCfgErrR = crate::BitReader;
#[doc = "Field `SR_PARAM_CFG_ERR` writer - The interrupt enable bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
pub type SrParamCfgErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_PARAM_CFG_ERR` reader - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
pub type BlendParamCfgErrR = crate::BitReader;
#[doc = "Field `BLEND_PARAM_CFG_ERR` writer - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
pub type BlendParamCfgErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn sr_eof(&self) -> SrEofR {
        SrEofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof(&self) -> BlendEofR {
        BlendEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn sr_param_cfg_err(&self) -> SrParamCfgErrR {
        SrParamCfgErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn blend_param_cfg_err(&self) -> BlendParamCfgErrR {
        BlendParamCfgErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn sr_eof(&mut self) -> SrEofW<'_, IntEnaSpec> {
        SrEofW::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof(&mut self) -> BlendEofW<'_, IntEnaSpec> {
        BlendEofW::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn sr_param_cfg_err(&mut self) -> SrParamCfgErrW<'_, IntEnaSpec> {
        SrParamCfgErrW::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn blend_param_cfg_err(&mut self) -> BlendParamCfgErrW<'_, IntEnaSpec> {
        BlendParamCfgErrW::new(self, 3)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
