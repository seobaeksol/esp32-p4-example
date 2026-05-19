#[doc = "Register `GEN_STMP_CFG` reader"]
pub type R = crate::R<GenStmpCfgSpec>;
#[doc = "Register `GEN_STMP_CFG` writer"]
pub type W = crate::W<GenStmpCfgSpec>;
#[doc = "Field `A_UPMETHOD` reader - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type AUpmethodR = crate::FieldReader;
#[doc = "Field `A_UPMETHOD` writer - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type AUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B_UPMETHOD` reader - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type BUpmethodR = crate::FieldReader;
#[doc = "Field `B_UPMETHOD` writer - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type BUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A_SHDW_FULL` reader - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
pub type AShdwFullR = crate::BitReader;
#[doc = "Field `A_SHDW_FULL` writer - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
pub type AShdwFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_SHDW_FULL` reader - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
pub type BShdwFullR = crate::BitReader;
#[doc = "Field `B_SHDW_FULL` writer - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
pub type BShdwFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn a_upmethod(&self) -> AUpmethodR {
        AUpmethodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn b_upmethod(&self) -> BUpmethodR {
        BUpmethodR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
    #[inline(always)]
    pub fn a_shdw_full(&self) -> AShdwFullR {
        AShdwFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
    #[inline(always)]
    pub fn b_shdw_full(&self) -> BShdwFullR {
        BShdwFullR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the update method for PWM generator %s time stamp A's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn a_upmethod(&mut self) -> AUpmethodW<'_, GenStmpCfgSpec> {
        AUpmethodW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures the update method for PWM generator %s time stamp B's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn b_upmethod(&mut self) -> BUpmethodW<'_, GenStmpCfgSpec> {
        BUpmethodW::new(self, 4)
    }
    #[doc = "Bit 8 - Represents whether or not generator%s time stamp A's shadow reg is transferred.\\\\0: A's active reg has been updated with shadow register latest value.\\\\1: A's shadow reg is filled and waiting to be transferred to A's active reg"]
    #[inline(always)]
    pub fn a_shdw_full(&mut self) -> AShdwFullW<'_, GenStmpCfgSpec> {
        AShdwFullW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents whether or not generator%s time stamp B's shadow reg is transferred.\\\\0: B's active reg has been updated with shadow register latest value.\\\\1: B's shadow reg is filled and waiting to be transferred to B's active reg"]
    #[inline(always)]
    pub fn b_shdw_full(&mut self) -> BShdwFullW<'_, GenStmpCfgSpec> {
        BShdwFullW::new(self, 9)
    }
}
#[doc = "Generator0 time stamp registers A and B transfer status and update method register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_stmp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenStmpCfgSpec;
impl crate::RegisterSpec for GenStmpCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_stmp_cfg::R`](R) reader structure"]
impl crate::Readable for GenStmpCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gen_stmp_cfg::W`](W) writer structure"]
impl crate::Writable for GenStmpCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN_STMP_CFG to value 0"]
impl crate::Resettable for GenStmpCfgSpec {}
