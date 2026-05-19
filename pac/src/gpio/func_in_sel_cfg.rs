#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FuncInSelCfgSpec>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FuncInSelCfgSpec>;
#[doc = "Field `IN_SEL` reader - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
pub type InSelR = crate::FieldReader;
#[doc = "Field `IN_SEL` writer - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
pub type InSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type InInvSelR = crate::BitReader;
#[doc = "Field `IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type InInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&self) -> InSelR {
        InSelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> InInvSelR {
        InInvSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
    #[inline(always)]
    pub fn in_sel(&mut self) -> InSelW<'_, FuncInSelCfgSpec> {
        InSelW::new(self, 0)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn in_inv_sel(&mut self) -> InInvSelW<'_, FuncInSelCfgSpec> {
        InInvSelW::new(self, 6)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, FuncInSelCfgSpec> {
        SelW::new(self, 7)
    }
}
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncInSelCfgSpec;
impl crate::RegisterSpec for FuncInSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FuncInSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FuncInSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0"]
impl crate::Resettable for FuncInSelCfgSpec {}
