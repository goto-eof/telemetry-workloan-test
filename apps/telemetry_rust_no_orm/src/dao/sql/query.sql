       SELECT
        telemetry_no_orm.value,
        COUNT(telemetry_no_orm.value) AS n_pcs
        FROM
        telemetry_no_orm
        WHERE
        telemetry_no_orm.property = $1
        AND telemetry_no_orm.request_id IN (
            SELECT
            MAX(telemetry_no_orm.request_id)
            FROM
            telemetry_no_orm
            WHERE
            telemetry_no_orm.property = $2
            GROUP BY
            property,
            value
        )
        GROUP BY
        telemetry_no_orm.code,
        telemetry_no_orm.property,
        telemetry_no_orm.value
        ORDER BY
        telemetry_no_orm.property ASC