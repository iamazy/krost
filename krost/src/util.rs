use crate::{Krost, KrostError};

fn version_check<T: Krost>(version: i16) -> Result<(), KrostError> {
    let version_added = <T as Krost>::version_added();
    let version_removed = <T as Krost>::version_removed();
    if version < version_added || version_removed.map(|vr| version > vr).unwrap_or(false) {
        Err(KrostError::InvalidVersion {
            min: version_added,
            max: version_removed,
            version,
        })
    } else {
        Ok(())
    }
}

pub fn version_check_read<T: Krost, D: std::io::Read>(
    version: i16,
    buf: &mut D,
) -> Result<T, KrostError> {
    version_check::<T>(version)?;
    T::decode(buf)
}
