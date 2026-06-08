use crate::plugin::{Plugin, PluginArch, PluginFormat};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS plugins (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                vendor TEXT NOT NULL DEFAULT '',
                version TEXT NOT NULL DEFAULT '',
                format TEXT NOT NULL,
                path TEXT NOT NULL,
                bundle_id TEXT NOT NULL DEFAULT '',
                arch TEXT NOT NULL DEFAULT 'Unknown',
                size_bytes INTEGER NOT NULL DEFAULT 0,
                enabled INTEGER NOT NULL DEFAULT 1,
                last_scanned TEXT DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_plugins_name ON plugins(name);
            CREATE INDEX IF NOT EXISTS idx_plugins_format ON plugins(format);",
        )?;
        Ok(())
    }

    pub fn upsert_plugins(&self, plugins: &[Plugin]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "INSERT INTO plugins (id, name, vendor, version, format, path, bundle_id, arch, size_bytes, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                vendor = excluded.vendor,
                version = excluded.version,
                format = excluded.format,
                path = excluded.path,
                bundle_id = excluded.bundle_id,
                arch = excluded.arch,
                size_bytes = excluded.size_bytes,
                enabled = excluded.enabled,
                last_scanned = CURRENT_TIMESTAMP",
        )?;

        for plugin in plugins {
            stmt.execute(params![
                plugin.id,
                plugin.name,
                plugin.vendor,
                plugin.version,
                plugin.format.label(),
                plugin.path,
                plugin.bundle_id,
                plugin.arch.label(),
                plugin.size_bytes as i64,
                plugin.enabled as i64,
            ])?;
        }

        Ok(())
    }

    pub fn get_all_plugins(&self) -> Result<Vec<Plugin>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, vendor, version, format, path, bundle_id, arch, size_bytes, enabled
             FROM plugins ORDER BY name ASC",
        )?;

        let plugins = stmt
            .query_map([], |row| {
                let format_str: String = row.get(4)?;
                let arch_str: String = row.get(7)?;

                let format = match format_str.as_str() {
                    "AU" => PluginFormat::AudioUnit,
                    "VST3" => PluginFormat::Vst3,
                    _ => PluginFormat::Vst3,
                };

                let arch = match arch_str.as_str() {
                    "Apple Silicon" => PluginArch::AppleSilicon,
                    "Intel 64" => PluginArch::Intel,
                    "Universal" => PluginArch::Universal,
                    _ => PluginArch::Unknown,
                };

                Ok(Plugin {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    vendor: row.get(2)?,
                    version: row.get(3)?,
                    format,
                    path: row.get(5)?,
                    bundle_id: row.get(6)?,
                    arch,
                    size_bytes: row.get::<_, i64>(8)? as u64,
                    enabled: row.get::<_, i64>(9)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(plugins)
    }

    pub fn update_enabled(&self, id: &str, enabled: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE plugins SET enabled = ?1 WHERE id = ?2",
            params![enabled as i64, id],
        )?;
        Ok(())
    }

    pub fn delete_plugin(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM plugins WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_plugin(&self, id: &str) -> Result<Option<Plugin>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, vendor, version, format, path, bundle_id, arch, size_bytes, enabled
             FROM plugins WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let format_str: String = row.get(4)?;
            let arch_str: String = row.get(7)?;

            let format = match format_str.as_str() {
                "AU" => PluginFormat::AudioUnit,
                "VST3" => PluginFormat::Vst3,
                _ => PluginFormat::Vst3,
            };

            let arch = match arch_str.as_str() {
                "Apple Silicon" => PluginArch::AppleSilicon,
                "Intel 64" => PluginArch::Intel,
                "Universal" => PluginArch::Universal,
                _ => PluginArch::Unknown,
            };

            Ok(Some(Plugin {
                id: row.get(0)?,
                name: row.get(1)?,
                vendor: row.get(2)?,
                version: row.get(3)?,
                format,
                path: row.get(5)?,
                bundle_id: row.get(6)?,
                arch,
                size_bytes: row.get::<_, i64>(8)? as u64,
                enabled: row.get::<_, i64>(9)? != 0,
            }))
        } else {
            Ok(None)
        }
    }
}
