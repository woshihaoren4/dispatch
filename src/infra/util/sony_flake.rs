use lazy_static::lazy_static;
use wd_sonyflake::SonyFlakeEntity;

lazy_static!(static ref SFE: SonyFlakeEntity = SonyFlakeEntity::new_default(););



pub fn sony_flake_id()->i64{
    SFE.get_id()
}