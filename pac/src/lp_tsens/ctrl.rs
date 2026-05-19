#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `OUT` reader - Temperature sensor data out."]
pub type OutR = crate::FieldReader;
#[doc = "Field `READY` reader - Indicate temperature sensor out ready."]
pub type ReadyR = crate::BitReader;
#[doc = "Field `SAMPLE_EN` reader - Enable sample signal for wakeup module."]
pub type SampleEnR = crate::BitReader;
#[doc = "Field `SAMPLE_EN` writer - Enable sample signal for wakeup module."]
pub type SampleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_MASK` reader - Wake up signal mask."]
pub type WakeupMaskR = crate::BitReader;
#[doc = "Field `WAKEUP_MASK` writer - Wake up signal mask."]
pub type WakeupMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - Enable temperature sensor to send out interrupt."]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - Enable temperature sensor to send out interrupt."]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_INV` reader - Invert temperature sensor data."]
pub type InInvR = crate::BitReader;
#[doc = "Field `IN_INV` writer - Invert temperature sensor data."]
pub type InInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DIV` reader - Temperature sensor clock divider."]
pub type ClkDivR = crate::FieldReader;
#[doc = "Field `CLK_DIV` writer - Temperature sensor clock divider."]
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POWER_UP` reader - Temperature sensor power up."]
pub type PowerUpR = crate::BitReader;
#[doc = "Field `POWER_UP` writer - Temperature sensor power up."]
pub type PowerUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWER_UP_FORCE` reader - 1: dump out & power up controlled by SW, 0: by FSM."]
pub type PowerUpForceR = crate::BitReader;
#[doc = "Field `POWER_UP_FORCE` writer - 1: dump out & power up controlled by SW, 0: by FSM."]
pub type PowerUpForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Temperature sensor data out."]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Indicate temperature sensor out ready."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable sample signal for wakeup module."]
    #[inline(always)]
    pub fn sample_en(&self) -> SampleEnR {
        SampleEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake up signal mask."]
    #[inline(always)]
    pub fn wakeup_mask(&self) -> WakeupMaskR {
        WakeupMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn in_inv(&self) -> InInvR {
        InInvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn power_up(&self) -> PowerUpR {
        PowerUpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: dump out & power up controlled by SW, 0: by FSM."]
    #[inline(always)]
    pub fn power_up_force(&self) -> PowerUpForceR {
        PowerUpForceR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Enable sample signal for wakeup module."]
    #[inline(always)]
    pub fn sample_en(&mut self) -> SampleEnW<'_, CtrlSpec> {
        SampleEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Wake up signal mask."]
    #[inline(always)]
    pub fn wakeup_mask(&mut self) -> WakeupMaskW<'_, CtrlSpec> {
        WakeupMaskW::new(self, 10)
    }
    #[doc = "Bit 12 - Enable temperature sensor to send out interrupt."]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<'_, CtrlSpec> {
        IntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Invert temperature sensor data."]
    #[inline(always)]
    pub fn in_inv(&mut self) -> InInvW<'_, CtrlSpec> {
        InInvW::new(self, 13)
    }
    #[doc = "Bits 14:21 - Temperature sensor clock divider."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<'_, CtrlSpec> {
        ClkDivW::new(self, 14)
    }
    #[doc = "Bit 22 - Temperature sensor power up."]
    #[inline(always)]
    pub fn power_up(&mut self) -> PowerUpW<'_, CtrlSpec> {
        PowerUpW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: dump out & power up controlled by SW, 0: by FSM."]
    #[inline(always)]
    pub fn power_up_force(&mut self) -> PowerUpForceW<'_, CtrlSpec> {
        PowerUpForceW::new(self, 23)
    }
}
#[doc = "Tsens configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x0001_9400"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0001_9400;
}
