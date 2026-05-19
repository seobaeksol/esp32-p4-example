#[doc = "Register `DATA%s` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA%s` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `TX_BYTE` reader - In reset mode, it is acceptance code register n with R/W Permission. In operation mode, it stores the nth byte information of the data to be transmitted under operating mode."]
pub type TxByteR = crate::FieldReader;
#[doc = "Field `TX_BYTE` writer - In reset mode, it is acceptance code register n with R/W Permission. In operation mode, it stores the nth byte information of the data to be transmitted under operating mode."]
pub type TxByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register n with R/W Permission. In operation mode, it stores the nth byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte(&self) -> TxByteR {
        TxByteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register n with R/W Permission. In operation mode, it stores the nth byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte(&mut self) -> TxByteW<'_, DataSpec> {
        TxByteW::new(self, 0)
    }
}
#[doc = "Data register %s.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA%s to value 0"]
impl crate::Resettable for DataSpec {}
