package lineCounter;

import java.io.IOException;
import java.util.*;
import org.apache.hadoop.fs.Path;
import org.apache.hadoop.io.*;
import org.apache.hadoop.mapred.*;

public class LineCounter {

	// MAPPER CODE
	public static class Map extends MapReduceBase implements Mapper<LongWritable, Text, Text, IntWritable> {

		private final static IntWritable one = new IntWritable(1);
//		private Text line = new Text();
		private final Text line = new Text("The total number of lines:");

		public void map(LongWritable key, Text value, OutputCollector<Text, IntWritable> output, Reporter reporter)
				throws IOException {
			//if do this, it will include every line
			output.collect(line, one);
			
			//if do this, it will exclude null or empty lines 
//			String lineVal = value.toString();
//			if (lineVal != null && !lineVal.isEmpty()) {
//				output.collect(line, one);
//			}
		}
	}

	// REDUCER CODE
	public static class Reduce extends MapReduceBase implements Reducer<Text, IntWritable, Text, IntWritable> {

		public void reduce(Text key, Iterator<IntWritable> values, OutputCollector<Text, IntWritable> output,
				Reporter reporter) throws IOException {
			int sum = 0;
			while (values.hasNext()) {
				sum += values.next().get();
			}
			output.collect(key, new IntWritable(sum));
		}
	}

	// DRIVER CODE
	public static void main(String[] args) throws Exception {

		JobConf conf = new JobConf(LineCounter.class);
		conf.setJobName("linecount");

		conf.setOutputKeyClass(Text.class);
		conf.setOutputValueClass(IntWritable.class);

		conf.setMapperClass(Map.class);
		conf.setCombinerClass(Reduce.class);
		conf.setReducerClass(Reduce.class);

		conf.setInputFormat(TextInputFormat.class);
		conf.setOutputFormat(TextOutputFormat.class);

		FileInputFormat.setInputPaths(conf, new Path(args[0]));
		FileOutputFormat.setOutputPath(conf, new Path(args[1]));

		JobClient.runJob(conf);
	}
}

/*
 * version 2
 */
//import java.io.IOException;
//import org.apache.hadoop.conf.Configuration;
//import org.apache.hadoop.fs.Path;
//import org.apache.hadoop.io.IntWritable;
//import org.apache.hadoop.io.Text;
//import org.apache.hadoop.mapreduce.Job;
//import org.apache.hadoop.mapreduce.Mapper;
//import org.apache.hadoop.mapreduce.Reducer;
//import org.apache.hadoop.mapreduce.lib.input.FileInputFormat;
//import org.apache.hadoop.mapreduce.lib.output.FileOutputFormat;

//public class LineCounter {
//	public static class LineCountMapper
//	extends Mapper<Object, Text, Text, IntWritable>{
//
//		public void map(Object key, Text value, Context context
//				) throws IOException, InterruptedException {
//
//			String line = value.toString();
//			if (line != null && !line.isEmpty())
//				context.write(new Text("Number of Lines:"), new IntWritable(1));
//		}
//	}
//
//	public static class LineCountReducer
//	extends Reducer<Text,IntWritable,Text,IntWritable> {
//		private IntWritable result = new IntWritable();
//
//		public void reduce(Text key, Iterable<IntWritable> values,
//				Context context
//				) throws IOException, InterruptedException {
//			int sum = 0;
//			for (IntWritable val : values) {
//				sum += val.get();
//			}
//			result.set(sum);
//			context.write(key, result);
//		}
//	}
//
//	public static void main(String[] args) throws Exception {
//		Configuration conf = new Configuration();
//		Job job = Job.getInstance(conf, "line count");
//		job.setJarByClass(LineCounter.class);
//		job.setMapperClass(LineCountMapper.class);
//		job.setCombinerClass(LineCountReducer.class);
//		job.setReducerClass(LineCountReducer.class);
//		job.setOutputKeyClass(Text.class);
//		job.setOutputValueClass(IntWritable.class);
//		FileInputFormat.addInputPath(job, new Path(args[0]));
//		FileOutputFormat.setOutputPath(job, new Path(args[1]));
//		System.exit(job.waitForCompletion(true) ? 0 : 1);
//	}
//}

/*
 * version 3
 */
//import java.io.IOException;
//import java.util.Iterator;
//import org.apache.hadoop.fs.Path;
//import org.apache.hadoop.io.IntWritable;
//import org.apache.hadoop.io.LongWritable;
//import org.apache.hadoop.io.Text;
//import org.apache.hadoop.mapred.FileInputFormat;
//import org.apache.hadoop.mapred.FileOutputFormat;
//import org.apache.hadoop.mapred.JobClient;
//import org.apache.hadoop.mapred.JobConf;
//import org.apache.hadoop.mapred.MapReduceBase;
//import org.apache.hadoop.mapred.Mapper;
//import org.apache.hadoop.mapred.OutputCollector;
//import org.apache.hadoop.mapred.Reducer;
//import org.apache.hadoop.mapred.Reporter;
//import org.apache.hadoop.mapred.TextInputFormat;
//import org.apache.hadoop.mapred.TextOutputFormat;
//
//public class LineCounter {
//    // Map
//    public static class Map extends MapReduceBase implements Mapper<LongWritable, Text, Text, IntWritable> {
//        private final static IntWritable one = new IntWritable(1);
//        private final Text line = new Text("The number of lines:");
//
//        public void map(LongWritable key, Text value, OutputCollector<Text, IntWritable> output, Reporter reporter)
//                throws IOException {
//            // Count each line as an object
//            output.collect(line, one);
//        }
//    }
//
//    // Reduce
//    public static class Reduce extends MapReduceBase implements Reducer<Text, IntWritable, Text, IntWritable> {
//        public void reduce(Text key, Iterator<IntWritable> values, OutputCollector<Text, IntWritable> output, Reporter reporter) throws IOException {
//            int sum = 0;
//            while (values.hasNext()) {
//                sum += values.next().get();
//            }
//            output.collect(key, new IntWritable(sum));
//        }
//    }
//
//    public static void main(String[] args) throws Exception {
//        if (args.length < 2) {
//            System.err.println("Usage: LineCount <inputPath> <outputPath>");
//            System.exit(1);
//        }
//        JobConf conf = new JobConf(LineCounter.class);
//        conf.setJobName("linecount");
//
//        conf.setOutputKeyClass(Text.class);
//        conf.setOutputValueClass(IntWritable.class);
//
//        conf.setMapperClass(LineCounter.Map.class);
//        conf.setCombinerClass(LineCounter.Reduce.class);
//        conf.setReducerClass(LineCounter.Reduce.class);
//
//        conf.setInputFormat(TextInputFormat.class);
//        conf.setOutputFormat(TextOutputFormat.class);
//
//        FileInputFormat.addInputPath(conf, new Path(args[0]));
//        FileOutputFormat.setOutputPath(conf, new Path(args[1]));
//
//        JobClient.runJob(conf);
//    }
//}
