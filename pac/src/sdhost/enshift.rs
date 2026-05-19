#[doc = "Register `ENSHIFT` reader"]
pub type R = crate::R<EnshiftSpec>;
#[doc = "Register `ENSHIFT` writer"]
pub type W = crate::W<EnshiftSpec>;
#[doc = "Field `ENABLE_SHIFT` reader - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type EnableShiftR = crate::FieldReader;
#[doc = "Field `ENABLE_SHIFT` writer - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type EnableShiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    pub fn enable_shift(&self) -> EnableShiftR {
        EnableShiftR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    pub fn enable_shift(&mut self) -> EnableShiftW<'_, EnshiftSpec> {
        EnableShiftW::new(self, 0)
    }
}
#[doc = "Enable Phase Shift register\n\nYou can [`read`](crate::Reg::read) this register and get [`enshift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enshift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnshiftSpec;
impl crate::RegisterSpec for EnshiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enshift::R`](R) reader structure"]
impl crate::Readable for EnshiftSpec {}
#[doc = "`write(|w| ..)` method takes [`enshift::W`](W) writer structure"]
impl crate::Writable for EnshiftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENSHIFT to value 0"]
impl crate::Resettable for EnshiftSpec {}
