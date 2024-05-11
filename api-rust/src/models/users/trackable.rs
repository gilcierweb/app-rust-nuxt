// # Track information about your user sign in. It tracks the following columns:
// #
// # * sign_in_count      - Increased every time a sign in is made (by form, openid, oauth)
// # * current_sign_in_at - A timestamp updated when the user signs in
// # * last_sign_in_at    - Holds the timestamp of the previous sign in
// # * current_sign_in_ip - The remote ip updated when the user sign in
// # * last_sign_in_ip    - Holds the remote ip of the previous sign in
// #

// module Trackable
// def self.required_fields(klass)
// [:current_sign_in_at, :current_sign_in_ip, :last_sign_in_at, :last_sign_in_ip, :sign_in_count]
// end
//
// def update_tracked_fields(request)
// old_current, new_current = self.current_sign_in_at, Time.now.utc
// self.last_sign_in_at     = old_current || new_current
// self.current_sign_in_at  = new_current
//
// old_current, new_current = self.current_sign_in_ip, extract_ip_from(request)
// self.last_sign_in_ip     = old_current || new_current
// self.current_sign_in_ip  = new_current
//
// self.sign_in_count ||= 0
// self.sign_in_count += 1
// end
//
// def update_tracked_fields!(request)
// # We have to check if the user is already persisted before running
// # `save` here because invalid users can be saved if we don't.
// # See https://github.com/heartcombo/devise/issues/4673 for more details.
// return if new_record?
//
// update_tracked_fields(request)
// save(validate: false)
// end
//
// protected
//
// def extract_ip_from(request)
// request.remote_ip
// end
//
// end

// use std::time::SystemTime;


//     pub mod models {
//         pub mod trackable {
//             pub fn required_fields(klass: &str) -> Vec<&'static str> {
//                 vec![
//                     "current_sign_in_at",
//                     "current_sign_in_ip",
//                     "last_sign_in_at",
//                     "last_sign_in_ip",
//                     "sign_in_count",
//                 ]
//             }
//
//             pub fn update_tracked_fields(
//                 &mut self,
//                 request: &http::Request<()>,
//             ) -> Result<(), Box<dyn std::error::Error>> {
//                 let old_current = self.current_sign_in_at;
//                 self.current_sign_in_at = Some(SystemTime::now().into());
//
//                 let old_current_ip = self.current_sign_in_ip;
//                 self.current_sign_in_ip = Some(extract_ip_from(request));
//
//                 if let Some(old_current) = old_current {
//                     self.last_sign_in_at = Some(old_current);
//                 } else {
//                     self.last_sign_in_at = self.current_sign_in_at;
//                 }
//
//                 if let Some(old_current_ip) = old_current_ip {
//                     self.last_sign_in_ip = Some(old_current_ip);
//                 } else {
//                     self.last_sign_in_ip = self.current_sign_in_ip;
//                 }
//
//                 self.sign_in_count = self.sign_in_count.unwrap_or(0) + 1;
//
//                 Ok(())
//             }
//
//             pub fn update_tracked_fields_save(
//                 &mut self,
//                 request: &http::Request<()>,
//             ) -> Result<(), Box<dyn std::error::Error>> {
//                 if self.is_new_record() {
//                     return Ok(());
//                 }
//
//                 self.update_tracked_fields(request)?;
//                 self.save(false)?;
//                 Ok(())
//             }
//
//             fn extract_ip_from(request: &http::Request<()>) -> Option<String> {
//                 request.remote_addr().map(|addr| addr.to_string())
//             }
//         }
//     }


