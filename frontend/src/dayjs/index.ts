import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import utc from "dayjs/plugin/utc";
import duration from "dayjs/plugin/duration";

dayjs.extend(relativeTime);
dayjs.extend(utc);
dayjs.extend(duration);

export default dayjs;
