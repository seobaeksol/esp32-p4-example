#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Field `CH1_SRC_MULTBLK_TYPE` reader - NA"]
pub type Ch1SrcMultblkTypeR = crate::FieldReader;
#[doc = "Field `CH1_SRC_MULTBLK_TYPE` writer - NA"]
pub type Ch1SrcMultblkTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_DST_MULTBLK_TYPE` reader - NA"]
pub type Ch1DstMultblkTypeR = crate::FieldReader;
#[doc = "Field `CH1_DST_MULTBLK_TYPE` writer - NA"]
pub type Ch1DstMultblkTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_RD_UID` reader - NA"]
pub type Ch1RdUidR = crate::FieldReader;
#[doc = "Field `CH1_WR_UID` reader - NA"]
pub type Ch1WrUidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn ch1_src_multblk_type(&self) -> Ch1SrcMultblkTypeR {
        Ch1SrcMultblkTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn ch1_dst_multblk_type(&self) -> Ch1DstMultblkTypeR {
        Ch1DstMultblkTypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 18:21 - NA"]
    #[inline(always)]
    pub fn ch1_rd_uid(&self) -> Ch1RdUidR {
        Ch1RdUidR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 25:28 - NA"]
    #[inline(always)]
    pub fn ch1_wr_uid(&self) -> Ch1WrUidR {
        Ch1WrUidR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn ch1_src_multblk_type(&mut self) -> Ch1SrcMultblkTypeW<'_, Cfg0Spec> {
        Ch1SrcMultblkTypeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn ch1_dst_multblk_type(&mut self) -> Ch1DstMultblkTypeW<'_, Cfg0Spec> {
        Ch1DstMultblkTypeW::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
