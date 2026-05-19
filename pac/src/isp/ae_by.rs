#[doc = "Register `AE_BY` reader"]
pub type R = crate::R<AeBySpec>;
#[doc = "Register `AE_BY` writer"]
pub type W = crate::W<AeBySpec>;
#[doc = "Field `AE_Y_BSIZE` reader - this field configures every block y size"]
pub type AeYBsizeR = crate::FieldReader<u16>;
#[doc = "Field `AE_Y_BSIZE` writer - this field configures every block y size"]
pub type AeYBsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `AE_Y_START` reader - this field configures first block start y address"]
pub type AeYStartR = crate::FieldReader<u16>;
#[doc = "Field `AE_Y_START` writer - this field configures first block start y address"]
pub type AeYStartW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - this field configures every block y size"]
    #[inline(always)]
    pub fn ae_y_bsize(&self) -> AeYBsizeR {
        AeYBsizeR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - this field configures first block start y address"]
    #[inline(always)]
    pub fn ae_y_start(&self) -> AeYStartR {
        AeYStartR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - this field configures every block y size"]
    #[inline(always)]
    pub fn ae_y_bsize(&mut self) -> AeYBsizeW<'_, AeBySpec> {
        AeYBsizeW::new(self, 0)
    }
    #[doc = "Bits 11:21 - this field configures first block start y address"]
    #[inline(always)]
    pub fn ae_y_start(&mut self) -> AeYStartW<'_, AeBySpec> {
        AeYStartW::new(self, 11)
    }
}
#[doc = "ae window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ae_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBySpec;
impl crate::RegisterSpec for AeBySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_by::R`](R) reader structure"]
impl crate::Readable for AeBySpec {}
#[doc = "`write(|w| ..)` method takes [`ae_by::W`](W) writer structure"]
impl crate::Writable for AeBySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AE_BY to value 0xd8"]
impl crate::Resettable for AeBySpec {
    const RESET_VALUE: u32 = 0xd8;
}
