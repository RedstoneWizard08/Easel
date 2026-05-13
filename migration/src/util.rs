use cruet::Inflector;
use loco_rs::schema::{
    array, array_null, array_uniq, big_integer, big_integer_null, big_integer_uniq, big_unsigned,
    big_unsigned_null, big_unsigned_uniq, binary, binary_len, binary_len_null, binary_len_uniq,
    binary_null, binary_uniq, blob, blob_null, blob_uniq, boolean, boolean_null, char, char_len,
    char_len_null, char_len_uniq, char_null, char_uniq, date, date_null, date_time, date_time_null,
    date_time_uniq, date_uniq, decimal, decimal_len, decimal_len_null, decimal_len_uniq,
    decimal_null, decimal_uniq, double, double_null, double_uniq, enum_type, enum_type_null,
    enum_type_null_with_default, enum_type_with_default, float, float_null, float_uniq, integer,
    integer_null, integer_uniq, interval, interval_null, interval_uniq, json, json_binary,
    json_binary_null, json_binary_uniq, json_null, json_uniq, money, money_null, money_uniq,
    pk_auto, pk_uuid, small_integer, small_integer_null, small_integer_uniq, small_unsigned,
    small_unsigned_null, small_unsigned_uniq, string, string_len, string_len_null, string_len_uniq,
    string_null, string_uniq, text, text_null, text_uniq, time, time_null, time_uniq, timestamp,
    timestamptz, timestamptz_null, unsigned, unsigned_null, unsigned_uniq, uuid, uuid_null,
    uuid_uniq, var_binary, var_binary_null, var_binary_uniq, varbit, varbit_null, varbit_uniq,
    ColType,
};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DatabaseBackend, Statement},
};

#[derive(Iden)]
enum GeneralIds {
    CreatedAt,
    UpdatedAt,
}

pub async fn create_table(
    m: &SchemaManager<'_>,
    table: &str,
    cols: &[(&str, ColType)],
    refs: &[(&str, &str)], // [(from_tbl, to_tbl), ...]
) -> Result<(), DbErr> {
    create_table_impl(m, table, cols, refs, false, true).await
}

fn normalize_table(table: &str) -> String {
    cruet::to_plural(table).to_snake_case()
}

fn reference_id(totbl: &str) -> String {
    format!("{}_id", cruet::to_singular(totbl).to_snake_case())
}

trait ToDefPub {
    fn to_def_pub(&self, name: impl IntoIden) -> ColumnDef;
}

impl ToDefPub for ColType {
    #[allow(clippy::too_many_lines)]
    fn to_def_pub(&self, name: impl IntoIden) -> ColumnDef {
        match self {
            Self::PkAuto => pk_auto(name),
            Self::PkUuid => pk_uuid(name),
            Self::CharLen(len) => char_len(name, *len),
            Self::CharLenNull(len) => char_len_null(name, *len),
            Self::CharLenUniq(len) => char_len_uniq(name, *len),
            Self::Char => char(name),
            Self::CharNull => char_null(name),
            Self::CharUniq => char_uniq(name),
            Self::StringLen(len) => string_len(name, *len),
            Self::StringLenNull(len) => string_len_null(name, *len),
            Self::StringLenUniq(len) => string_len_uniq(name, *len),
            Self::String => string(name),
            Self::StringNull => string_null(name),
            Self::StringUniq => string_uniq(name),
            Self::Text => text(name),
            Self::TextNull => text_null(name),
            Self::TextUniq => text_uniq(name),
            Self::Integer => integer(name),
            Self::IntegerNull => integer_null(name),
            Self::IntegerUniq => integer_uniq(name),
            // Self::TinyInteger => tiny_integer(name),
            // Self::TinyIntegerNull => tiny_integer_null(name),
            // Self::TinyIntegerUniq => tiny_integer_uniq(name),
            Self::Unsigned => unsigned(name),
            Self::UnsignedNull => unsigned_null(name),
            Self::UnsignedUniq => unsigned_uniq(name),
            // Self::TinyUnsigned => tiny_unsigned(name),
            // Self::TinyUnsignedNull => tiny_unsigned_null(name),
            // Self::TinyUnsignedUniq => tiny_unsigned_uniq(name),
            Self::SmallUnsigned => small_unsigned(name),
            Self::SmallUnsignedNull => small_unsigned_null(name),
            Self::SmallUnsignedUniq => small_unsigned_uniq(name),
            Self::BigUnsigned => big_unsigned(name),
            Self::BigUnsignedNull => big_unsigned_null(name),
            Self::BigUnsignedUniq => big_unsigned_uniq(name),
            Self::SmallInteger => small_integer(name),
            Self::SmallIntegerNull => small_integer_null(name),
            Self::SmallIntegerUniq => small_integer_uniq(name),
            Self::BigInteger => big_integer(name),
            Self::BigIntegerNull => big_integer_null(name),
            Self::BigIntegerUniq => big_integer_uniq(name),
            Self::Decimal => decimal(name),
            Self::DecimalNull => decimal_null(name),
            Self::DecimalUniq => decimal_uniq(name),
            Self::DecimalLen(precision, scale) => decimal_len(name, *precision, *scale),
            Self::DecimalLenNull(precision, scale) => decimal_len_null(name, *precision, *scale),
            Self::DecimalLenUniq(precision, scale) => decimal_len_uniq(name, *precision, *scale),
            Self::Float => float(name),
            Self::FloatNull => float_null(name),
            Self::FloatUniq => float_uniq(name),
            Self::Double => double(name),
            Self::DoubleNull => double_null(name),
            Self::DoubleUniq => double_uniq(name),
            Self::Boolean => boolean(name),
            Self::BooleanNull => boolean_null(name),
            // Self::Timestamp => timestamp(name),
            // Self::TimestampNull => timestamp_null(name),
            // Self::TimestampUniq => timestamp_uniq(name),
            Self::Date => date(name),
            Self::DateNull => date_null(name),
            Self::DateUniq => date_uniq(name),
            Self::DateTime => date_time(name),
            Self::DateTimeNull => date_time_null(name),
            Self::DateTimeUniq => date_time_uniq(name),
            Self::Time => time(name),
            Self::TimeNull => time_null(name),
            Self::TimeUniq => time_uniq(name),
            Self::Interval(ival, prec) => interval(name, ival.clone(), *prec),
            Self::IntervalNull(ival, prec) => interval_null(name, ival.clone(), *prec),
            Self::IntervalUniq(ival, prec) => interval_uniq(name, ival.clone(), *prec),
            Self::Binary => binary(name),
            Self::BinaryNull => binary_null(name),
            Self::BinaryUniq => binary_uniq(name),
            Self::BinaryLen(len) => binary_len(name, *len),
            Self::BinaryLenNull(len) => binary_len_null(name, *len),
            Self::BinaryLenUniq(len) => binary_len_uniq(name, *len),
            Self::VarBinary(len) => var_binary(name, *len),
            Self::VarBinaryNull(len) => var_binary_null(name, *len),
            Self::VarBinaryUniq(len) => var_binary_uniq(name, *len),
            Self::TimestampWithTimeZone => timestamptz(name),
            Self::TimestampWithTimeZoneNull => timestamptz_null(name),
            Self::Json => json(name),
            Self::JsonNull => json_null(name),
            Self::JsonUniq => json_uniq(name),
            Self::JsonBinary => json_binary(name),
            Self::JsonBinaryNull => json_binary_null(name),
            Self::JsonBinaryUniq => json_binary_uniq(name),
            Self::Blob => blob(name),
            Self::BlobNull => blob_null(name),
            Self::BlobUniq => blob_uniq(name),
            Self::Money => money(name),
            Self::MoneyNull => money_null(name),
            Self::MoneyUniq => money_uniq(name),
            Self::Uuid => uuid(name),
            Self::UuidNull => uuid_null(name),
            Self::UuidUniq => uuid_uniq(name),
            Self::VarBitLen(len) => varbit(name, *len),
            Self::VarBitLenNull(len) => varbit_null(name, *len),
            Self::VarBitLenUniq(len) => varbit_uniq(name, *len),
            Self::Array(kind) => array(name, kind.clone()),
            Self::ArrayNull(kind) => array_null(name, kind.clone()),
            Self::ArrayUniq(kind) => array_uniq(name, kind.clone()),
            // Enum types
            Self::Enum(enum_name, _) => enum_type(name, enum_name),
            Self::EnumNull(enum_name, _) => enum_type_null(name, enum_name),
            Self::EnumWithDefault(enum_name, _, default_value) => {
                enum_type_with_default(name, enum_name, default_value)
            }
            Self::EnumNullWithDefault(enum_name, _, default_value) => {
                enum_type_null_with_default(name, enum_name, default_value)
            }
            // defaults
            Self::MoneyWithDefault(v) => money(name).default(*v).take(),
            Self::IntegerWithDefault(v) => integer(name).default(*v).take(),
            Self::UnsignedWithDefault(v) => unsigned(name).default(*v).take(),
            Self::SmallUnsignedWithDefault(v) => small_unsigned(name).default(*v).take(),
            Self::BigUnsignedWithDefault(v) => big_unsigned(name).default(*v).take(),
            Self::SmallIntegerWithDefault(v) => small_integer(name).default(*v).take(),
            Self::BigIntegerWithDefault(v) => big_integer(name).default(*v).take(),
            Self::DecimalWithDefault(v) => decimal(name).default(*v).take(),
            Self::DecimalLenWithDefault(p, s, v) => decimal_len(name, *p, *s).default(*v).take(),
            Self::FloatWithDefault(v) => float(name).default(*v).take(),
            Self::DoubleWithDefault(v) => double(name).default(*v).take(),
            Self::BooleanWithDefault(v) => boolean(name).default(*v).take(),
            Self::DateWithDefault(v) => date(name).default(v.clone()).take(),
            Self::DateTimeWithDefault(v) => date_time(name).default(v.clone()).take(),
            Self::TimeWithDefault(v) => time(name).default(v.clone()).take(),
            Self::TimestampWithTimeZoneWithDefault(v) => {
                timestamptz(name).default(v.clone()).take()
            }
            Self::CharWithDefault(v) => char(name).default(*v).take(),
            Self::CharLenWithDefault(len, v) => char_len(name, *len).default(*v).take(),
            Self::StringWithDefault(v) => string(name).default(v.clone()).take(),
            Self::StringLenWithDefault(len, v) => string_len(name, *len).default(v.clone()).take(),
            Self::TextWithDefault(v) => text(name).default(v.clone()).take(),
        }
    }
}

async fn create_table_impl(
    m: &SchemaManager<'_>,
    table: &str,
    cols: &[(&str, ColType)],
    refs: &[(&str, &str)], // [(from_tbl, to_tbl), ...]
    is_join: bool,
    add_timestamps: bool, // New parameter to control timestamp addition
) -> Result<(), DbErr> {
    let nz_table = normalize_table(table);

    // Create enum types automatically if they don't exist
    let mut enum_types = std::collections::HashSet::new();
    for (_, col_type) in cols {
        match col_type {
            ColType::Enum(enum_name, variants)
            | ColType::EnumNull(enum_name, variants)
            | ColType::EnumWithDefault(enum_name, variants, _)
            | ColType::EnumNullWithDefault(enum_name, variants, _) => {
                if !enum_types.contains(enum_name) {
                    enum_types.insert(enum_name.clone());

                    // Check if enum type already exists
                    let enum_exists = check_enum_exists(m, enum_name).await?;

                    if !enum_exists {
                        // Create enum type with provided variants
                        match m.get_database_backend() {
                            sea_orm::DatabaseBackend::Postgres => {
                                let variant_aliases: Vec<Alias> =
                                    variants.iter().map(Alias::new).collect();
                                m.create_type(
                                    sea_query::extension::postgres::Type::create()
                                        .as_enum(Alias::new(enum_name))
                                        .values(variant_aliases)
                                        .to_owned(),
                                )
                                .await?;
                            }
                            #[allow(clippy::match_same_arms)]
                            sea_orm::DatabaseBackend::Sqlite => {
                                // SQLite doesn't support native enum types
                                // The enum behavior will be handled by the column definition
                                // which will create a TEXT column with CHECK constraints
                            }
                            sea_orm::DatabaseBackend::MySql => {
                                // MySql not supporting
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Conditionally create table with or without timestamps
    let mut stmt = if add_timestamps {
        table_auto_tz(Alias::new(&nz_table))
    } else {
        Table::create()
            .table(Alias::new(&nz_table))
            .if_not_exists()
            .take()
    };

    if is_join {
        let mut idx = Index::create();
        idx.name(format!("idx-{nz_table}-refs-pk"))
            .table(Alias::new(&nz_table));

        for (from_tbl, ref_name) in refs {
            let nz_from_table = normalize_table(from_tbl);
            // in movies, user:references, creates a `user_id` field or what ever in
            // `ref_name` if given
            let nz_ref_name = if ref_name.is_empty() {
                reference_id(&nz_from_table)
            } else {
                (*ref_name).to_string()
            };
            idx.col(Alias::new(nz_ref_name));
        }
        stmt.primary_key(&mut idx);
    }

    for (name, atype) in cols {
        stmt.col(atype.to_def_pub(Alias::new(*name)));
    }

    // user, None
    // users, None
    // user, admin_id
    for (from_tbl, ref_name) in refs {
        // Check for nullable reference
        let (nz_from_table, is_nullable) = from_tbl.strip_suffix('?').map_or_else(
            || (normalize_table(from_tbl), false),
            |stripped| (normalize_table(stripped), true),
        );
        let nz_ref_name = if ref_name.is_empty() {
            reference_id(&nz_from_table)
        } else {
            (*ref_name).to_string()
        };
        // Only add the column if it doesn't already exist in cols
        if !cols.iter().any(|(col_name, _)| *col_name == nz_ref_name) {
            let col_type = if is_nullable {
                ColType::IntegerNull
            } else {
                ColType::Integer
            };
            stmt.col(col_type.to_def_pub(Alias::new(&nz_ref_name)));
        }
        // Set FK actions based on nullability
        let mut fk = ForeignKey::create();
        fk.name(format!("fk-{nz_from_table}-{nz_ref_name}-to-{nz_table}"));
        fk.from(Alias::new(&nz_table), Alias::new(&nz_ref_name));
        fk.to(Alias::new(nz_from_table), Alias::new("id"));
        if is_nullable {
            fk.on_delete(ForeignKeyAction::SetNull);
            fk.on_update(ForeignKeyAction::NoAction);
        } else {
            fk.on_delete(ForeignKeyAction::Cascade);
            fk.on_update(ForeignKeyAction::Cascade);
        }
        stmt.foreign_key(&mut fk);
    }
    m.create_table(stmt).await?;
    Ok(())
}

pub fn table_auto_tz<T>(name: T) -> TableCreateStatement
where
    T: IntoIden + 'static,
{
    timestamps_tz(Table::create().table(name).if_not_exists().take())
}

#[must_use]
pub fn timestamps_tz(t: TableCreateStatement) -> TableCreateStatement {
    let mut t = t;
    t.col(timestamp(GeneralIds::CreatedAt).default(Expr::current_timestamp()))
        .col(timestamp(GeneralIds::UpdatedAt).default(Expr::current_timestamp()));
    t.take()
}

async fn check_enum_exists(m: &SchemaManager<'_>, enum_name: &str) -> Result<bool, DbErr> {
    match m.get_database_backend() {
        DatabaseBackend::Postgres => {
            let query = format!(
                "SELECT EXISTS (
                    SELECT 1 FROM pg_type 
                    WHERE typname = '{enum_name}' 
                    AND typtype = 'e'
                )"
            );

            let result = m
                .get_connection()
                .query_one(Statement::from_string(DatabaseBackend::Postgres, query))
                .await?;

            Ok(result.is_some_and(|row| row.try_get::<bool>("", "exists").unwrap_or(false)))
        }
        DatabaseBackend::Sqlite => {
            // SQLite doesn't have native enum types, so we'll always return false
            // to allow creation of enum-like behavior through CHECK constraints
            Ok(false)
        }
        DatabaseBackend::MySql => {
            // MySQL doesn't support enums in the same way, so we'll always return false
            Ok(false)
        }
    }
}
