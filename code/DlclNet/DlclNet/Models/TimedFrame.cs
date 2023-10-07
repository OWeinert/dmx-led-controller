using System.Collections;

namespace DlclNet.Models;

public struct TimedFrame: IFrame
{
    public IEnumerable<Pixel> Pixels { get; set; }
    public uint FrameTime { get; set; }

    public IEnumerable<SingleFrame> Split()
    {
        var sFrames = new List<SingleFrame>();
        for (var i = 0; i < FrameTime; i++)
        {
            sFrames.Add(new SingleFrame
            {
                Pixels = Pixels
            });
        }
        return sFrames;
    }
}