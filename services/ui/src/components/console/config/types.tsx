export enum Operation {
	LIST,
	ADD,
	VIEW,
	EDIT,
	DELETE,
	PERF,
	BILLING,
	HELP,
}

export enum Button {
	ADD,
	INVITE,
	REFRESH,
	BACK,
}

export enum Resource {
	ORGANIZATIONS,
	MEMBERS,
	BILLING,
	PROJECTS,
	REPORTS,
	METRIC_KINDS,
	BRANCHES,
	TESTBEDS,
	BENCHMARKS,
	THRESHOLDS,
	ALERTS,
	USERS,
	TOKENS,
	HELP,
}

export enum Row {
	TEXT,
	BOOL,
	SELECT,
	FOREIGN,
}

export enum Card {
	FIELD,
	TABLE,
}

export enum Display {
	RAW,
	SWITCH,
	SELECT,
}

export enum PerfTab {
	BRANCHES = "branches",
	TESTBEDS = "testbeds",
	BENCHMARKS = "benchmarks",
}

export const isPerfTab = (tab: string) =>
	tab === PerfTab.BRANCHES ||
	tab === PerfTab.TESTBEDS ||
	tab === PerfTab.BENCHMARKS;

export enum Range {
	DATE_TIME = "date_time",
	VERSION = "version",
}

export const is_range = (range: string) =>
	range === Range.DATE_TIME || range === Range.VERSION;
