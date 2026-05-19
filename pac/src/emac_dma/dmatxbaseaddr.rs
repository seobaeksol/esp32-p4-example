#[doc = "Register `DMATXBASEADDR` reader"]
pub type R = crate::R<DmatxbaseaddrSpec>;
#[doc = "Register `DMATXBASEADDR` writer"]
pub type W = crate::W<DmatxbaseaddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxbaseaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxbaseaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatxbaseaddrSpec;
impl crate::RegisterSpec for DmatxbaseaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxbaseaddr::R`](R) reader structure"]
impl crate::Readable for DmatxbaseaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatxbaseaddr::W`](W) writer structure"]
impl crate::Writable for DmatxbaseaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATXBASEADDR to value 0"]
impl crate::Resettable for DmatxbaseaddrSpec {}
