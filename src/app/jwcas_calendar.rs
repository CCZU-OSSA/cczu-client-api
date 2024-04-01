use super::{jwcas::JwcasApplication, jwcas_calendar_type::{ClassInfo, Schedule, EVENT_PROP}};
use regex::Regex;
use scraper::{Html, Selector};
use std::{collections::HashMap, future::Future};
use uuid::Uuid;

pub trait JwcasApplicationCalendarExt {
    fn get_class_icalendar(&self) -> impl Future<Output = Result<Vec<ClassInfo>, String>>;
}

impl<'a> JwcasApplicationCalendarExt for JwcasApplication<'a> {
    async fn get_class_icalendar(&self) -> Result<Vec<ClassInfo>, String> {
        let text = self.get_classlist_html().await.unwrap();

        let doc = Html::parse_document(&text);
        let tb_up_seletor = Selector::parse(r#"table[id="GVxkall"]"#).unwrap();
        let tb_dn_seletor = Selector::parse(r#"table[id="GVxkkb"]"#).unwrap();
        let tb_up_itemseletor =
            Selector::parse(r#"tr[class="dg1-item"] > td[style="width:20%;"]"#).unwrap();
        let tb_dn_rowseletor = Selector::parse(r#"tr[class="dg1-item"]"#).unwrap();
        let tb_dn_itemseletor = Selector::parse(r#"td[style="width:12%;"]"#).unwrap();
        let class_namelist: Vec<String> = doc
            .select(&tb_up_seletor)
            .next()
            .unwrap()
            .select(&tb_up_itemseletor)
            .map(|e| e.inner_html())
            .collect();

        let row_matrix: Vec<Vec<String>> = doc
            .select(&tb_dn_seletor)
            .next()
            .unwrap()
            .select(&tb_dn_rowseletor)
            .map(|e| {
                e.select(&tb_dn_itemseletor)
                    .map(|item| item.inner_html())
                    .collect()
            })
            .collect();
        let mut column_matrix: Vec<Vec<String>> = vec![];
        for i in 0..7 {
            let mut tmp: Vec<String> = vec![];
            for v in row_matrix.iter() {
                tmp.push(v[i].clone())
            }
            column_matrix.push(tmp.clone());
        }

        let mut course_info: HashMap<String, ClassInfo> = HashMap::new();

        for (day, courses) in column_matrix.iter().enumerate() {
            for (time, course_cb) in courses.iter().enumerate() {
                let mut target: Vec<String> = course_cb
                    .split("/")
                    .filter(|v| !v.is_empty())
                    .map(|v| v.to_string())
                    .collect();
                let targetlen = target.len();
                for index in 0..targetlen {
                    let course = target[index].clone();
                    if course != "&nbsp;" {
                        let id = Uuid::new_v3(
                            &Uuid::NAMESPACE_DNS,
                            format!("{}{}", course, day).as_bytes(),
                        )
                        .to_string();

                        if !course_info.contains_key(&id) || time == 0 {
                            let nl: Vec<String> = class_namelist
                                .iter()
                                .filter(|e| course.starts_with(e.as_str()))
                                .map(|e| e.clone())
                                .collect();
                            if nl.is_empty() {
                                if index < targetlen - 1 {
                                    target[index + 1] =
                                        format!("{}/{}", course.clone(), target[index + 1]);
                                    continue;
                                }
                                return Err(format!("Unable to resolve course name correctly"));
                            }

                            let classname = nl[0].clone();

                            let re = Regex::new(r#"(\S+)? *([单双]?) *((\d+-\d+,?)+)"#).unwrap();
                            let pattern = course.replace(&classname, "").trim().to_string();
                            let Some(data) = re.captures(pattern.as_str()) else {
                                panic!("Course information parsing abnormal")
                            }; //'X立德楼409  7-8,'

                            let info = ClassInfo::new(
                                classname,
                                match data.get(2).map_or("", |m| m.as_str()) {
                                    "单" => 1,
                                    "双" => 2,
                                    _ => 3,
                                },
                                day + 1,
                                data.get(3)
                                    .map_or("", |m| m.as_str())
                                    .split(",")
                                    .filter(|e| !e.is_empty())
                                    .map(|e| e.to_string())
                                    .collect(),
                                vec![time + 1],
                                data.get(1)
                                    .map_or(String::new(), |m| m.as_str().to_string()),
                            );
                            course_info.insert(id, info);
                        } else {
                            course_info.get_mut(&id).unwrap().add_classtime(time + 1);
                        }
                    }
                }
            }
        }

        Ok(course_info.values().map(|e| e.clone()).collect())
    }
}

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use icalendar::{Alarm, Calendar, Component, Event, EventLike, Trigger};

pub struct ICal {
    pub firstweekdate: String,
    pub schedule: Schedule,
    pub classlist: Vec<ClassInfo>,
}

pub fn get_reminder(reminder: &str) -> Option<i32> {
    if reminder.is_empty() {
        None
    } else {
        Some(reminder.parse::<i32>().unwrap_or(15))
    }
}

impl ICal {
    pub fn new(firstweekdate: String, classlist: Vec<ClassInfo>) -> Self {
        Self {
            firstweekdate,
            schedule: Schedule::get_schedule(),
            classlist,
        }
    }

    pub fn to_ical(&mut self, reminder: Option<i32>) -> Calendar {
        let mut cal = Calendar::new();
        cal.timezone("Asia/Shanghai").name("课程表");
        self.classlist.iter_mut().for_each(|e| {
            e.with_startdate(&self.firstweekdate);
        });

        for info in self.classlist.iter() {
            let start_time = self.schedule.classtime[info.classtime.first().unwrap() - 1]
                .clone()
                .start_time;
            let end_time = self.schedule.classtime[info.classtime.last().unwrap() - 1]
                .clone()
                .end_time;
            let create_time = Utc::now();
            for day in info.daylist.iter() {
                let uid = format!("{}@gmail.com", Uuid::new_v4());
                let start = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, start_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();
                let end = NaiveDateTime::parse_from_str(
                    format!("{}{}", day, end_time).as_str(),
                    "%Y%m%d%H%M",
                )
                .unwrap();

                let mut event = Event::new();

                EVENT_PROP.iter().for_each(|(k, v)| {
                    event.add_property(k, v);
                });

                event
                    .summary(&info.name)
                    .location(&info.classroom)
                    .timestamp(create_time)
                    .uid(&uid)
                    .starts(start)
                    .ends(end);
                if let Some(reminder) = reminder {
                    event.alarm(Alarm::display(
                        "课前提醒",
                        Trigger::before_start(Duration::minutes(reminder as i64)),
                    ));
                }

                cal.push(event);
            }
        }

        // week

        let mut fweek = NaiveDateTime::new(
            NaiveDate::parse_from_str(&self.firstweekdate.clone(), "%Y%m%d").unwrap(),
            NaiveTime::default(),
        );

        let create_time = Utc::now();
        for wn in 1..=19 {
            let summary = format!("学期第 {} 周", wn);
            let uid = format!("{}@gmail.com", Uuid::new_v4());
            let mut event = Event::new();
            event
                .uid(&uid)
                .summary(&summary)
                .timestamp(create_time)
                .starts(fweek.date())
                .ends(fweek.date() + Duration::days(7));

            EVENT_PROP.iter().for_each(|(k, v)| {
                event.add_property(k, v);
            });

            cal.push(event.clone());
            fweek += Duration::days(7);
        }

        cal
    }
}
