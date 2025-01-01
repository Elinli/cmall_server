use jwt_simple::prelude::*;

use crate::User;

type JwtError = jwt_simple::Error;

const JWT_DURATION: u64 = 7 * 24 * 60 * 60;
const JWT_ISSUER: &str = "cmall_server";
const JWT_AUDIENCE: &str = "cmall_frontend";

#[derive(Clone)]
pub struct EncodingKeyPair(Ed25519KeyPair);
#[derive(Debug, Clone)]
pub struct DecodingKeyPair(Ed25519PublicKey);

impl EncodingKeyPair {
    pub fn load_secret_key(secret_key: &str) -> Result<Self, JwtError> {
        let key_pair = Ed25519KeyPair::from_pem(secret_key)?;
        Ok(Self(key_pair))
    }

    pub fn sign(&self, user: impl Into<User>) -> Result<String, JwtError> {
        let claims = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = claims.with_issuer(JWT_ISSUER).with_audience(JWT_AUDIENCE);
        self.0.sign(claims)
    }
}

impl DecodingKeyPair {
    pub fn load_public_key(public_key: &str) -> Result<Self, JwtError> {
        let key_pair = Ed25519PublicKey::from_pem(public_key)?;
        Ok(Self(key_pair))
    }

    pub fn verify(&self, token: &str) -> Result<User, JwtError> {
        let options = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISSUER])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUDIENCE])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(options))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod test_jwt {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_jwt_should_work() -> Result<()> {
        let secret_key = include_str!("../../fixtures/private.pem");
        let public_key = include_str!("../../fixtures/public.pem");

        let encoding_key_pair = EncodingKeyPair::load_secret_key(secret_key).unwrap();
        let decoding_key_pair = DecodingKeyPair::load_public_key(public_key)?;

        let user = User::new(1, "Eli Shi", "elixy@qq.com", "138");

        let token = encoding_key_pair.sign(user.clone()).unwrap();
        // assert_eq!(token, "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MSwibmFtZSI6IkVsaSBTaGkiLCJlbWFpbCI6ImVsaXh5QHFxLmNvbSIsInBob25lIjoiMTM4IiwiaXN");

        let decoded_user = decoding_key_pair.verify(&token)?;

        assert_eq!(decoded_user.username, user.username);
        Ok(())
    }
}
