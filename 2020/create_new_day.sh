current_day=`date +'%d'`
folder_name="day-$current_day"
mkdir $folder_name
cp template/template.py $folder_name/$folder_name.py
cd $folder_name
touch input.txt
echo "Created new folder and files for day $current_day"