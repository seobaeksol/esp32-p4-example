#[doc = "Register `IN_PERI_SEL` reader"]
pub type R = crate::R<InPeriSelSpec>;
#[doc = "Register `IN_PERI_SEL` writer"]
pub type W = crate::W<InPeriSelSpec>;
#[doc = "Field `PERI_IN_SEL` reader - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
pub type PeriInSelR = crate::FieldReader;
#[doc = "Field `PERI_IN_SEL` writer - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
pub type PeriInSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
    #[inline(always)]
    pub fn peri_in_sel(&self) -> PeriInSelR {
        PeriInSelR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to select peripheral for Rx channel 0. 0:lcdcam. 1: gpspi_2. 2: gpspi_3. 3: parl_io. 4: aes. 5: sha. 6~15: Dummy"]
    #[inline(always)]
    pub fn peri_in_sel(&mut self) -> PeriInSelW<'_, InPeriSelSpec> {
        PeriInSelW::new(self, 0)
    }
}
#[doc = "Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InPeriSelSpec;
impl crate::RegisterSpec for InPeriSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_peri_sel::R`](R) reader structure"]
impl crate::Readable for InPeriSelSpec {}
#[doc = "`write(|w| ..)` method takes [`in_peri_sel::W`](W) writer structure"]
impl crate::Writable for InPeriSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_PERI_SEL to value 0x3f"]
impl crate::Resettable for InPeriSelSpec {
    const RESET_VALUE: u32 = 0x3f;
}
