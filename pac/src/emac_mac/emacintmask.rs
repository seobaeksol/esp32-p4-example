#[doc = "Register `EMACINTMASK` reader"]
pub type R = crate::R<EmacintmaskSpec>;
#[doc = "Register `EMACINTMASK` writer"]
pub type W = crate::W<EmacintmaskSpec>;
#[doc = "Field `PMTINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PmtintmaskR = crate::BitReader;
#[doc = "Field `PMTINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PmtintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LpiintmaskR = crate::BitReader;
#[doc = "Field `LPIINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LpiintmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtintmask(&self) -> PmtintmaskR {
        PmtintmaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiintmask(&self) -> LpiintmaskR {
        LpiintmaskR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtintmask(&mut self) -> PmtintmaskW<'_, EmacintmaskSpec> {
        PmtintmaskW::new(self, 3)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiintmask(&mut self) -> LpiintmaskW<'_, EmacintmaskSpec> {
        LpiintmaskW::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`emacintmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacintmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmacintmaskSpec;
impl crate::RegisterSpec for EmacintmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacintmask::R`](R) reader structure"]
impl crate::Readable for EmacintmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`emacintmask::W`](W) writer structure"]
impl crate::Writable for EmacintmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACINTMASK to value 0"]
impl crate::Resettable for EmacintmaskSpec {}
