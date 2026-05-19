#[doc = "Register `INT_ST0` reader"]
pub type R = crate::R<IntSt0Spec>;
#[doc = "Field `ACK_WITH_ERR_0` reader - NA"]
pub type AckWithErr0R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_1` reader - NA"]
pub type AckWithErr1R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_2` reader - NA"]
pub type AckWithErr2R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_3` reader - NA"]
pub type AckWithErr3R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_4` reader - NA"]
pub type AckWithErr4R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_5` reader - NA"]
pub type AckWithErr5R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_6` reader - NA"]
pub type AckWithErr6R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_7` reader - NA"]
pub type AckWithErr7R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_8` reader - NA"]
pub type AckWithErr8R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_9` reader - NA"]
pub type AckWithErr9R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_10` reader - NA"]
pub type AckWithErr10R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_11` reader - NA"]
pub type AckWithErr11R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_12` reader - NA"]
pub type AckWithErr12R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_13` reader - NA"]
pub type AckWithErr13R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_14` reader - NA"]
pub type AckWithErr14R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_15` reader - NA"]
pub type AckWithErr15R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_0` reader - NA"]
pub type DphyErrors0R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_1` reader - NA"]
pub type DphyErrors1R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_2` reader - NA"]
pub type DphyErrors2R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_3` reader - NA"]
pub type DphyErrors3R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_4` reader - NA"]
pub type DphyErrors4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ack_with_err_0(&self) -> AckWithErr0R {
        AckWithErr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ack_with_err_1(&self) -> AckWithErr1R {
        AckWithErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ack_with_err_2(&self) -> AckWithErr2R {
        AckWithErr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ack_with_err_3(&self) -> AckWithErr3R {
        AckWithErr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ack_with_err_4(&self) -> AckWithErr4R {
        AckWithErr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ack_with_err_5(&self) -> AckWithErr5R {
        AckWithErr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ack_with_err_6(&self) -> AckWithErr6R {
        AckWithErr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn ack_with_err_7(&self) -> AckWithErr7R {
        AckWithErr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn ack_with_err_8(&self) -> AckWithErr8R {
        AckWithErr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn ack_with_err_9(&self) -> AckWithErr9R {
        AckWithErr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn ack_with_err_10(&self) -> AckWithErr10R {
        AckWithErr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ack_with_err_11(&self) -> AckWithErr11R {
        AckWithErr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ack_with_err_12(&self) -> AckWithErr12R {
        AckWithErr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ack_with_err_13(&self) -> AckWithErr13R {
        AckWithErr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ack_with_err_14(&self) -> AckWithErr14R {
        AckWithErr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn ack_with_err_15(&self) -> AckWithErr15R {
        AckWithErr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dphy_errors_0(&self) -> DphyErrors0R {
        DphyErrors0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dphy_errors_1(&self) -> DphyErrors1R {
        DphyErrors1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn dphy_errors_2(&self) -> DphyErrors2R {
        DphyErrors2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dphy_errors_3(&self) -> DphyErrors3R {
        DphyErrors3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn dphy_errors_4(&self) -> DphyErrors4R {
        DphyErrors4R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSt0Spec;
impl crate::RegisterSpec for IntSt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st0::R`](R) reader structure"]
impl crate::Readable for IntSt0Spec {}
#[doc = "`reset()` method sets INT_ST0 to value 0"]
impl crate::Resettable for IntSt0Spec {}
