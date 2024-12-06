use std::future::Future;

use common::Core;
use store::write::now;

use crate::SpamFilterContext;

pub trait SpamFilterAnalyzeEhlo: Sync + Send {
    fn spam_filter_analyze_date(
        &self,
        ctx: &mut SpamFilterContext<'_>,
    ) -> impl Future<Output = ()> + Send;
}

impl SpamFilterAnalyzeEhlo for Core {
    async fn spam_filter_analyze_date(&self, ctx: &mut SpamFilterContext<'_>) {
        if let Some(date) = ctx.input.message.date() {
            let date = date.to_timestamp();
            if date != 0 {
                let date_diff = now() as i64 - date;

                if date_diff > 86400 {
                    // Older than a day
                    ctx.add_tag("DATE_IN_PAST");
                } else if -date_diff > 7200 {
                    //# More than 2 hours in the future
                    ctx.add_tag("DATE_IN_FUTURE");
                }
            } else {
                ctx.add_tag("INVALID_DATE");
            }
        } else {
            ctx.add_tag("MISSING_DATE");
        }
    }
}
