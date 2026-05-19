#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `CH1_AR_PROT` reader - NA"]
pub type Ch1ArProtR = crate::FieldReader;
#[doc = "Field `CH1_AR_PROT` writer - NA"]
pub type Ch1ArProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_AW_PROT` reader - NA"]
pub type Ch1AwProtR = crate::FieldReader;
#[doc = "Field `CH1_AW_PROT` writer - NA"]
pub type Ch1AwProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_ARLEN_EN` reader - NA"]
pub type Ch1ArlenEnR = crate::BitReader;
#[doc = "Field `CH1_ARLEN_EN` writer - NA"]
pub type Ch1ArlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ARLEN` reader - NA"]
pub type Ch1ArlenR = crate::FieldReader;
#[doc = "Field `CH1_ARLEN` writer - NA"]
pub type Ch1ArlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH1_AWLEN_EN` reader - NA"]
pub type Ch1AwlenEnR = crate::BitReader;
#[doc = "Field `CH1_AWLEN_EN` writer - NA"]
pub type Ch1AwlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_AWLEN` reader - NA"]
pub type Ch1AwlenR = crate::FieldReader;
#[doc = "Field `CH1_AWLEN` writer - NA"]
pub type Ch1AwlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH1_SRC_STAT_EN` reader - NA"]
pub type Ch1SrcStatEnR = crate::BitReader;
#[doc = "Field `CH1_SRC_STAT_EN` writer - NA"]
pub type Ch1SrcStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_DST_STAT_EN` reader - NA"]
pub type Ch1DstStatEnR = crate::BitReader;
#[doc = "Field `CH1_DST_STAT_EN` writer - NA"]
pub type Ch1DstStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_IOC_BLKTFR` reader - NA"]
pub type Ch1IocBlktfrR = crate::BitReader;
#[doc = "Field `CH1_IOC_BLKTFR` writer - NA"]
pub type Ch1IocBlktfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_LAST` reader - NA"]
pub type Ch1ShadowregOrLliLastR = crate::BitReader;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_LAST` writer - NA"]
pub type Ch1ShadowregOrLliLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_VALID` reader - NA"]
pub type Ch1ShadowregOrLliValidR = crate::BitReader;
#[doc = "Field `CH1_SHADOWREG_OR_LLI_VALID` writer - NA"]
pub type Ch1ShadowregOrLliValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_ar_prot(&self) -> Ch1ArProtR {
        Ch1ArProtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch1_aw_prot(&self) -> Ch1AwProtR {
        Ch1AwProtR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_arlen_en(&self) -> Ch1ArlenEnR {
        Ch1ArlenEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch1_arlen(&self) -> Ch1ArlenR {
        Ch1ArlenR::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch1_awlen_en(&self) -> Ch1AwlenEnR {
        Ch1AwlenEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch1_awlen(&self) -> Ch1AwlenR {
        Ch1AwlenR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch1_src_stat_en(&self) -> Ch1SrcStatEnR {
        Ch1SrcStatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_dst_stat_en(&self) -> Ch1DstStatEnR {
        Ch1DstStatEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch1_ioc_blktfr(&self) -> Ch1IocBlktfrR {
        Ch1IocBlktfrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_last(&self) -> Ch1ShadowregOrLliLastR {
        Ch1ShadowregOrLliLastR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_valid(&self) -> Ch1ShadowregOrLliValidR {
        Ch1ShadowregOrLliValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_ar_prot(&mut self) -> Ch1ArProtW<'_, Ctl1Spec> {
        Ch1ArProtW::new(self, 0)
    }
    #[doc = "Bits 3:5 - NA"]
    #[inline(always)]
    pub fn ch1_aw_prot(&mut self) -> Ch1AwProtW<'_, Ctl1Spec> {
        Ch1AwProtW::new(self, 3)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_arlen_en(&mut self) -> Ch1ArlenEnW<'_, Ctl1Spec> {
        Ch1ArlenEnW::new(self, 6)
    }
    #[doc = "Bits 7:14 - NA"]
    #[inline(always)]
    pub fn ch1_arlen(&mut self) -> Ch1ArlenW<'_, Ctl1Spec> {
        Ch1ArlenW::new(self, 7)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ch1_awlen_en(&mut self) -> Ch1AwlenEnW<'_, Ctl1Spec> {
        Ch1AwlenEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn ch1_awlen(&mut self) -> Ch1AwlenW<'_, Ctl1Spec> {
        Ch1AwlenW::new(self, 16)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn ch1_src_stat_en(&mut self) -> Ch1SrcStatEnW<'_, Ctl1Spec> {
        Ch1SrcStatEnW::new(self, 24)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_dst_stat_en(&mut self) -> Ch1DstStatEnW<'_, Ctl1Spec> {
        Ch1DstStatEnW::new(self, 25)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn ch1_ioc_blktfr(&mut self) -> Ch1IocBlktfrW<'_, Ctl1Spec> {
        Ch1IocBlktfrW::new(self, 26)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_last(&mut self) -> Ch1ShadowregOrLliLastW<'_, Ctl1Spec> {
        Ch1ShadowregOrLliLastW::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_shadowreg_or_lli_valid(&mut self) -> Ch1ShadowregOrLliValidW<'_, Ctl1Spec> {
        Ch1ShadowregOrLliValidW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
