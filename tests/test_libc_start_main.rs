use inline_c::assert_c;

#[test]
fn test_minimal() {
    (assert_c! {
        int main() {
            return 0;
        }
    })
    .success();
}
