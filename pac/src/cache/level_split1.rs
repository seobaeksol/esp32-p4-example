#[doc = "Register `LEVEL_SPLIT1` reader"]
pub type R = crate::R<LevelSplit1Spec>;
#[doc = "Field `LEVEL_SPLIT1` reader - Reserved"]
pub type LevelSplit1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn level_split1(&self) -> LevelSplit1R {
        LevelSplit1R::new(self.bits)
    }
}
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE\n\nYou can [`read`](crate::Reg::read) this register and get [`level_split1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelSplit1Spec;
impl crate::RegisterSpec for LevelSplit1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level_split1::R`](R) reader structure"]
impl crate::Readable for LevelSplit1Spec {}
#[doc = "`reset()` method sets LEVEL_SPLIT1 to value 0x03d0"]
impl crate::Resettable for LevelSplit1Spec {
    const RESET_VALUE: u32 = 0x03d0;
}
