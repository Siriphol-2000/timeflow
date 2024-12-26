use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::FirstName))
                    .col(string(Users::LastName))
                    .col(string(Users::Role))
                    .col(string(Users::Email))
                    .col(timestamp_with_time_zone(Users::CreatedAt))
                    .col(timestamp_with_time_zone(Users::UpdatedAt))
                    .to_owned(),
            ).await;
            manager .create_table(
                Table::create()
                .table(Sites::Table)
                .if_not_exists()
                .col(pk_auto(Sites::Id))
                .col(string(Sites::CustomerName))
                .col(string(Sites::ClientSite))
                .col(timestamp_with_time_zone(Sites::CreatedAt))
                .col(timestamp_with_time_zone(Sites::UpdatedAt))
                .to_owned(),
            ).await;
            manager.create_table(
                Table::create()
                .table(TimeSheet::Table)
                .if_not_exists()
                .col(pk_auto(TimeSheet::Id))
                .col(timestamp_with_time_zone(TimeSheet::TimeStart))
                .col(timestamp_with_time_zone(TimeSheet::TimeEnd))
                .col(string(TimeSheet::Activity))
                .col(float(TimeSheet::WorkingDay))
                .col(float(TimeSheet::LeavingDay))
                .col(timestamp_with_time_zone(TimeSheet::CreatedAt))
                .col(timestamp_with_time_zone(TimeSheet::UpdatedAt))
                .col(integer(TimeSheet::UsersId).not_null())
                .col(integer(TimeSheet::SitesId).not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-users")
                        .from(TimeSheet::Table,TimeSheet::UsersId)     
                        .to(Users::Table,Users::Id)
                )
                .foreign_key(
                    ForeignKey::create()
                    .name("fk-sites")
                    .from(TimeSheet::Table,TimeSheet::SitesId)     
                    .to(Sites::Table,Sites::Id)
                )
                .to_owned(),
            )
            .await;
        manager.create_table(
            Table::create()
            .table(UsersSites::Table)
            .if_not_exists()
            .col(pk_auto(UsersSites::Id))
            .col(integer(UsersSites::UsersId).not_null())
            .col(integer(UsersSites::SitesId).not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk-users")
                    .from(UsersSites::Table,UsersSites::UsersId)     
                    .to(Users::Table,Users::Id)
            )
            .foreign_key(
                ForeignKey::create()
                .name("fk-sites")
                .from(UsersSites::Table,UsersSites::SitesId)     
                .to(Sites::Table,Sites::Id)
            )
            .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
    manager.drop_table(Table::drop().table(UsersSites::Table).to_owned()).await;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await;
        manager.drop_table(Table::drop().table(Sites::Table).to_owned())
            .await;
        manager.drop_table(Table::drop().table(TimeSheet::Table).to_owned())
            .await

    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Role,
    Email,
    CreatedAt,
    UpdatedAt,
}


#[derive(DeriveIden)]
enum Sites {
    Table,
    Id,
    CustomerName,
    ClientSite,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TimeSheet{
    Table,
    Id,
    TimeStart,
    TimeEnd,
    Activity,
    WorkingDay,
    LeavingDay,
    CreatedAt,
    UpdatedAt,
    UsersId,
    SitesId,
}
#[derive(DeriveIden)]
enum UsersSites{
    Table,
    Id,
    UsersId,
    SitesId
}