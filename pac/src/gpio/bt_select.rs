#[doc = "Register `BT_SELECT` reader"]
pub type R = crate::R<BtSelectSpec>;
#[doc = "Register `BT_SELECT` writer"]
pub type W = crate::W<BtSelectSpec>;
#[doc = "Field `BT_SEL` reader - GPIO bit select register"]
pub type BtSelR = crate::FieldReader<u32>;
#[doc = "Field `BT_SEL` writer - GPIO bit select register"]
pub type BtSelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO bit select register"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BtSelR {
        BtSelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO bit select register"]
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BtSelW<'_, BtSelectSpec> {
        BtSelW::new(self, 0)
    }
}
#[doc = "GPIO bit select register\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtSelectSpec;
impl crate::RegisterSpec for BtSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_select::R`](R) reader structure"]
impl crate::Readable for BtSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`bt_select::W`](W) writer structure"]
impl crate::Writable for BtSelectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BT_SELECT to value 0"]
impl crate::Resettable for BtSelectSpec {}
