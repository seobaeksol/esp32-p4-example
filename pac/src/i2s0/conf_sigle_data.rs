#[doc = "Register `CONF_SIGLE_DATA` reader"]
pub type R = crate::R<ConfSigleDataSpec>;
#[doc = "Register `CONF_SIGLE_DATA` writer"]
pub type W = crate::W<ConfSigleDataSpec>;
#[doc = "Field `SINGLE_DATA` reader - The configured constant channel data to be sent out."]
pub type SingleDataR = crate::FieldReader<u32>;
#[doc = "Field `SINGLE_DATA` writer - The configured constant channel data to be sent out."]
pub type SingleDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    pub fn single_data(&self) -> SingleDataR {
        SingleDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The configured constant channel data to be sent out."]
    #[inline(always)]
    pub fn single_data(&mut self) -> SingleDataW<'_, ConfSigleDataSpec> {
        SingleDataW::new(self, 0)
    }
}
#[doc = "I2S signal data register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf_sigle_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf_sigle_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSigleDataSpec;
impl crate::RegisterSpec for ConfSigleDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_sigle_data::R`](R) reader structure"]
impl crate::Readable for ConfSigleDataSpec {}
#[doc = "`write(|w| ..)` method takes [`conf_sigle_data::W`](W) writer structure"]
impl crate::Writable for ConfSigleDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for ConfSigleDataSpec {}
